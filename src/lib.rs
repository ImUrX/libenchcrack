extern crate strum;
#[macro_use] extern crate strum_macros;
pub mod utils;
pub mod manipulation;

use crate::utils::SimpleRandom;
use wasm_bindgen::prelude::*;
use std::ops::Range;
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

#[wasm_bindgen]
pub struct Manipulator {
    player_seed: u64,
    item: manipulation::Item,
    wanted: Vec<manipulation::EnchantmentInstance>,
    unwanted: Vec<manipulation::EnchantmentInstance>
}

#[wasm_bindgen]
impl Manipulator {
    #[wasm_bindgen]
    pub fn new(seed1: u32, seed2: u32, item: manipulation::Item) -> Option<Manipulator> {
        let seed1_high = ((seed1 as u64) << 16) & 0x0000_FFFF_FFFF_0000;
        let seed2_high = ((seed2 as u64) << 16) & 0x0000_FFFF_FFFF_0000;

        for seed1_low in 0..65536 {
            let part: u64 = (Wrapping(seed1_high | seed1_low) * Wrapping(0x5DEECE66D) + Wrapping(0xB)).0;
            if (part & 0x0000_FFFF_FFFF_0000) == seed2_high {
                return Some(Self {
                    player_seed: part & 0x0000_FFFF_FFFF_FFFF,
                    wanted: Vec::new(),
                    unwanted: Vec::new(), item
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
    pub fn want(&mut self, ench: manipulation::EnchantmentInstance) {
        self.wanted.push(ench);
    }

    #[wasm_bindgen(js_name = notWant)]
    pub fn not_want(&mut self, ench: manipulation::EnchantmentInstance) {
        self.unwanted.push(ench);
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.wanted.clear();
        self.unwanted.clear();
    }

    #[wasm_bindgen]
    pub fn simulate(&mut self, max_shelves: i32, player_level: i32, version: manipulation::Version) -> Option<js_sys::Int32Array> {
        let mut seed = self.player_seed;
        let array = js_sys::Int32Array::new_with_length(3);
        if self.wanted.is_empty() {
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
                    let mut level = manipulation::Enchantment::calc_enchantment_table_level(&mut rand, j, bookshelves, self.item);
                    if level < j + 1{
                        level = 0;
                    }
                    enchant_levels[j as usize] = level;
                }
                
                'slotLoop: for j in 0..3 {
                    slot = j as i32;
                    // Get enchantments (changes RNG seed)
                    let enchantments = manipulation::Enchantment::get_enchantments_in_table(&mut rand, xp_seed as i32, self.item, j as i32, enchant_levels[j], version);
                    
                    if enchant_levels[j] == 0 {
                        continue 'slotLoop;
                    } else if i == -1 && player_level < enchant_levels[j] {
                        continue 'slotLoop
                    } else if player_level < enchant_levels[j] + 1 {
                        continue 'slotLoop
                    }

                    // Does this list contain all the enchantments we want?
                    for wanted_ench in &self.wanted {
                        let mut found = false;
                        for ench in &enchantments {
                            if wanted_ench.enchantment() != ench.enchantment() { continue; }
                            if wanted_ench.level() > ench.level() { continue 'slotLoop; }
                            found = true;
                            break;
                        }
                        if !found { continue 'slotLoop; }
                    }

                    for unwanted_ench in &self.unwanted {
                        for ench in &enchantments {
                            if unwanted_ench.enchantment() == ench.enchantment() { continue 'slotLoop; }
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

    #[wasm_bindgen]
    pub fn update(&mut self, times_needed: i32, chosen_slot: i32, player_level: i32) -> i32 {
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
}
