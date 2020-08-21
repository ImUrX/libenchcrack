extern crate web_sys;
use web_sys::console;
use core::cmp;
use core::num::Wrapping;

const MULT: i64 = 0x5DEECE66D;
const MASK: i64 = (1 << 48) - 1;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}


pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}


pub struct Rand {
    pub seed: i64,
}

impl Rand {
    pub fn new() -> Rand {
        Rand { seed: 0 }
    }

    pub fn set_seed(&mut self, seed: i64) {
        self.seed = (seed ^ MULT) & MASK;
    }

    pub fn next(&mut self) -> i32 {
        self.seed = (Wrapping(self.seed) * Wrapping(MULT) + Wrapping(0xB)).0 & MASK;
        (self.seed as u64 >> 17) as i32
    }
    
    pub fn next_int(&mut self, bound: Wrapping<i32>) -> i32 {
        let mut r = Wrapping(self.next());
        let m = bound - Wrapping(1);
        if (bound & m) == Wrapping(0) {
            r = Wrapping(((bound.0 as i64 * r.0 as i64) >> 31) as i32);
        } else {
            let mut u = r;
            while {r = u % bound; u - r + m} < Wrapping(0) {
                u = Wrapping(self.next());
            }
        }
        r.0
    }

    fn generic_enchantibility(&mut self, shelves: i32) -> i32 {
        let first = self.next_int(Wrapping(8));
        let second = self.next_int(Wrapping(shelves + 1));
        first + 1 + (shelves >> 1) + second
    }

    pub fn levels_slot1(&mut self, shelves: i32) -> i32 {
        let slot1 = self.generic_enchantibility(shelves) / 3;
        if slot1 < 1 { 1 } else { slot1 }
    }

    pub fn levels_slot2(&mut self, shelves: i32) -> i32 {
        (self.generic_enchantibility(shelves) * 2 / 3) + 1
    }

    pub fn levels_slot3(&mut self, shelves: i32) -> i32 {
        cmp::max(self.generic_enchantibility(shelves), shelves * 2)
    }

    pub fn verify_seed(&mut self, seed: i32, shelves: i32, slot1: i32, slot2: i32, slot3: i32) -> bool {
        self.set_seed(seed as i64);
        self.levels_slot1(shelves) == slot1 && self.levels_slot2(shelves) == slot2 && self.levels_slot3(shelves) == slot3
    }
}
