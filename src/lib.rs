extern crate strum;
#[macro_use] extern crate strum_macros;
pub mod utils;
pub mod manipulation;

use crate::utils::SimpleRandom;
use crate::manipulation::*;
use wasm_bindgen::prelude::*;
use std::ops::Range;
use enum_map::{enum_map, EnumMap};
use std::num::Wrapping;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Cracker {
    possible_seeds: Vec<i32>,
    start_size: Range<i32>,
    thread_id: usize,
    threads: usize,
    rng: SimpleRandom
}

#[wasm_bindgen]
impl Cracker {

    #[wasm_bindgen(constructor)]
    pub fn new(thread_id: usize, threads: usize) -> Self {
        let size = u32::MAX / threads as u32;
        let start = (i32::MIN as i64 + (size * thread_id as u32) as i64) as i32;
        Cracker {
            possible_seeds: Vec::with_capacity((80e6 as usize) / threads),
            start_size: 
                start..(start as i64 + size as i64) as i32 
                + if thread_id == threads - 1 { thread_id as i32 } else { 0 } ,
            rng: SimpleRandom::new(),
            thread_id, threads
        }
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.possible_seeds.clear();
    }

    #[wasm_bindgen(getter = possibleSeeds)]
    pub fn possible_seeds(&self) -> usize {
        self.possible_seeds.len()
    }

    #[wasm_bindgen(getter)]
    pub fn seed(&self) -> i32 {
        self.possible_seeds[0]
    }

    #[wasm_bindgen(js_name = firstInput)]
    pub fn first_input(&mut self, shelves: i32, slot1: i32, slot2: i32, slot3: i32,
        shelves_s: i32, slot_s1: i32, slot_s2: i32, slot_s3: i32) {    
        for seed in self.start_size.clone() {
            if self.rng.verify_seed(seed, shelves, slot1, slot2, slot3) 
            && self.rng.verify_seed(seed, shelves_s, slot_s1, slot_s2, slot_s3) { self.possible_seeds.push(seed); }
        }

        if self.thread_id == self.threads - 1 {
            if self.rng.verify_seed(i32::MAX, shelves, slot1, slot2, slot3)
            && self.rng.verify_seed(i32::MAX, shelves_s, slot_s1, slot_s2, slot_s3) { self.possible_seeds.push(i32::MAX); }
        }
    }

    #[wasm_bindgen(js_name = addInput)]
    pub fn add_input(&mut self, shelves: i32, slot1: i32, slot2: i32, slot3: i32) {
        let rng = &mut self.rng;
        self.possible_seeds.retain(|&x|
            rng.verify_seed(x, shelves, slot1, slot2, slot3) 
        );
    }

    pub fn contains(&self, x: i32) -> bool {
        self.possible_seeds.iter().any(|&y| x == y)
    }
}

pub struct ItemInstance {
    enchantments: Vec<EnchantmentInstance>,
}

impl ItemInstance {
    pub fn new() -> Self {
        Self {
            enchantments: Vec::new(),
        }
    }

    pub fn update(&mut self, ench: &EnchantmentInstance) {
        let opt = self.enchantments.iter().position(|x| *x == *ench);
        match opt {
            Some(index) => self.enchantments[index].level = ench.level,
            None => self.enchantments.push(ench.clone())
        }
    }
}

#[wasm_bindgen]
pub struct Manipulator {
    player_seed: u64,
    items: EnumMap<Item, ItemInstance>
}

#[wasm_bindgen]
impl Manipulator {
    #[wasm_bindgen]
    pub fn new(seed1: u32, seed2: u32) -> Option<Manipulator> {
        let seed1_high = ((seed1 as u64) << 16) & 0x0000_FFFF_FFFF_0000;
        let seed2_high = ((seed2 as u64) << 16) & 0x0000_FFFF_FFFF_0000;

        for seed1_low in 0..65536 {
            let part: u64 = (Wrapping(seed1_high | seed1_low) * Wrapping(0x5DEECE66D) + Wrapping(0xB)).0;
            if (part & 0x0000_FFFF_FFFF_0000) == seed2_high {
                return Some(Self {
                    player_seed: part & 0x0000_FFFF_FFFF_FFFF,
                    items: enum_map! {
                        _ => ItemInstance::new(),
                    }
                })
            }
        }
        None
    }

    #[wasm_bindgen(getter = playerSeed)]
    pub fn player_seed(&self) -> js_sys::Uint8Array {
        let array = js_sys::Uint8Array::new_with_length(6);
        let bytes = self.player_seed.to_le_bytes();
        for i in (0..array.length()).rev() {
            array.set_index(i, bytes[i as usize]);
        }
        array
    }

    #[wasm_bindgen]
    pub fn simulate(&mut self, item: Item, max_shelves: i32, player_level: i32, version: Version) -> Option<js_sys::Int32Array> {
        let mut seed = self.player_seed;
        let array = js_sys::Int32Array::new_with_length(3);
        if self.items[item].enchantments.is_empty() {
            return None;
        }
        // same as original EnchCracker
        // -2: not found; -1: no dummy enchantment needed; >= 0: number of times needed
		// to throw out item before dummy enchantment
        let mut times_needed = -2;
        let mut bookshelves_needed = 0;
        let mut slot = 0;
        let mut enchant_levels = [0; 3];

        'outerLoop: for i in -1..=(64*32) {
            let xp_seed = if i == -1 {
                // XP seed will be the current seed, because there is no dummy enchant
                seed >> 16
            } else {
                // XP seed will be the current seed, advanced by one because of the dummy enchant
                ((Wrapping(seed) * Wrapping(0x5DEECE66D) + Wrapping(0xB)).0 & 0x0000_FFFF_FFFF_FFFF) >> 16
            };
            let mut rand = java_rand::Random::new(0);
            for bookshelves in 0..=max_shelves {
                bookshelves_needed = bookshelves;
                rand.set_seed(xp_seed);

                //Calculate all slot levels
                for j in 0..3 {
                    let mut level = Enchantment::calc_enchantment_table_level(&mut rand, j, bookshelves, item);
                    if level < j + 1{
                        level = 0;
                    }
                    enchant_levels[j as usize] = level;
                }
                
                'slotLoop: for j in 0..3 {
                    slot = j as i32;
                    // Get enchantments (changes RNG seed)
                    let enchantments = Enchantment::get_enchantments_in_table(&mut rand, xp_seed as i32, item, j as i32, enchant_levels[j], version);
                    
                    if enchant_levels[j] == 0 || (i == -1 && player_level < enchant_levels[j]) || (player_level < enchant_levels[j] + 1) {
                        continue 'slotLoop;
                    }

                    // Does this list contain all the enchantments we want?
                    // I ended up changing it a little, level -1 means not wanted
                    for ench in self.items[item].enchantments.iter() {
                        if ench.level == -1 { continue; }
                        let mut found = false;
                        for found_ench in enchantments.iter() {
                            if ench.enchantment != found_ench.enchantment { continue; } 
                            if ench.level > found_ench.level { continue 'slotLoop; }
                            found = true;
                            break;
                        }
                        if !found { continue 'slotLoop; }
                    }

                    for ench in self.items[item].enchantments.iter() {
                        if ench.level != -1 { continue; }
                        for found_ench in enchantments.iter() {
                            if ench.enchantment == found_ench.enchantment { continue 'slotLoop; }
                        }
                    }

                    times_needed = i;
                    break 'outerLoop;
                }
            }

            //Simulate item throws
            if i != -1 {
                for _j in 0..4 {
                    seed = (Wrapping(seed) * Wrapping(0x5DEECE66D) + Wrapping(0xB)).0 & 0x0000_FFFF_FFFF_FFFF;
                }
            }
        }

        array.set_index(0, times_needed);
        array.set_index(1, bookshelves_needed);
        array.set_index(2, slot + 1);
        Some(array)
    }

    #[wasm_bindgen(js_name = updateSeed)]
    pub fn update_seed(&mut self, times_needed: i32, chosen_slot: i32, player_level: i32) -> i32 {
        if times_needed == -2 || chosen_slot == -1 {
            return player_level;
        }

        if times_needed != -1 {
            //items thrown
            for _i in 0..times_needed {
                for _j in 0..4 {
                    self.player_seed = (Wrapping(self.player_seed) * Wrapping(0x5DEECE66D) + Wrapping(0xB)).0 & 0x0000_FFFF_FFFF_FFFF;
                }
            }
            //dummy enchantment
            self.player_seed = (Wrapping(self.player_seed) * Wrapping(0x5DEECE66D) + Wrapping(0xB)).0 & 0x0000_FFFF_FFFF_FFFF;
        }
        //actual enchantment
        self.player_seed = (Wrapping(self.player_seed) * Wrapping(0x5DEECE66D) + Wrapping(0xB)).0 & 0x0000_FFFF_FFFF_FFFF;

        let mut player_level = player_level;
        if times_needed != -1 {
            player_level -= 1;
        }
        player_level - (chosen_slot + 1)
    }

    #[wasm_bindgen(js_name = updateItem)]
    pub fn update_item(&mut self, item: Item, ench: &EnchantmentInstance) {
        self.items[item].update(ench);
    }
}
