pub mod utils;

use crate::utils::Rand;
use wasm_bindgen::prelude::*;
use std::ops::Range;

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
    rng: Rand
}

#[wasm_bindgen]
impl Cracker {

    #[wasm_bindgen(constructor)]
    pub fn new(thread_id: usize, threads: usize) -> Cracker {
        let size = u32::MAX / threads as u32;
        let start = (i32::MIN as i64 + (size * thread_id as u32) as i64) as i32;
        Cracker {
            possible_seeds: Vec::with_capacity((80e6 as usize) / threads),
            start_size: 
                start..(start as i64 + size as i64) as i32 
                + if thread_id == threads - 1 { thread_id as i32 } else { 0 } ,
            rng: Rand::new(),
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
    pub fn seed(&mut self) -> String {
        format!("{:#010X}", self.possible_seeds[0]).split_off(2)
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
    player_seed: u64
}

const MIDDLE: u64 = 0x0000_ffff_ffff_0000;
#[wasm_bindgen]
impl Manipulator {

    #[wasm_bindgen(constructor)]
    pub fn new(seed1: u32, seed2: u32) -> Result<Manipulator, JsValue> {
        let seed1_high = ((seed1 as u64) << 16) & MIDDLE;
        let seed2_high = ((seed2 as u64) << 16) & MIDDLE;
        for seed1_low in 0..65536 {
            let possible_seed = ((seed1_high | seed1_low) * 0x5DEECE66D + 0xB) & MIDDLE;
            if possible_seed == seed2_high {
                return Ok(Manipulator {
                    player_seed: possible_seed
                });
            }
        }
        Err(js_sys::RangeError::new("Coulnd't find an XP Seed").into())
    }

    #[wasm_bindgen(getter = playerSeed)]
    pub fn player_seed(&self) -> String { // its this that increments size by 5kb or not supporting safari
        format!("{:#014X}", self.player_seed).split_off(2) // until v14 which is currently on beta (BigInt)
    }

    
}
