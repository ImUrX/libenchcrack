use std::cmp;
use std::num::Wrapping;

const MULT: i64 = 0x5DEECE66D;
const MASK: i64 = (1 << 48) - 1;

#[derive(Default)]
pub struct SimpleRandom {
    pub seed: i64,
}

impl SimpleRandom {
    pub fn set_seed(&mut self, seed: i64) {
        self.seed = (seed ^ MULT) & MASK;
    }

    pub fn next_int(&mut self) -> i32 {
        self.seed = (Wrapping(self.seed) * Wrapping(MULT) + Wrapping(0xB)).0 & MASK;
        (self.seed as u64 >> 17) as i32
    }

    pub fn next_int_bound(&mut self, bound: Wrapping<i32>) -> i32 {
        let mut r = Wrapping(self.next_int());
        let m = bound - Wrapping(1);
        if (bound & m) == Wrapping(0) {
            r = Wrapping(((bound.0 as i64 * r.0 as i64) >> 31) as i32);
        } else {
            let mut u = r;
            while {
                r = u % bound;
                u - r + m
            }
            .0 < 0
            {
                u = Wrapping(self.next_int());
            }
        }
        r.0
    }

    fn generic_enchantibility(&mut self, shelves: i32) -> i32 {
        let first = self.next_int_bound(Wrapping(8));
        let second = self.next_int_bound(Wrapping(shelves + 1));
        first + 1 + (shelves >> 1) + second
    }

    fn levels_slot1(&mut self, shelves: i32) -> i32 {
        let slot1 = self.generic_enchantibility(shelves) / 3;
        if slot1 < 1 {
            1
        } else {
            slot1
        }
    }

    fn levels_slot2(&mut self, shelves: i32) -> i32 {
        (self.generic_enchantibility(shelves) * 2 / 3) + 1
    }

    fn levels_slot3(&mut self, shelves: i32) -> i32 {
        cmp::max(self.generic_enchantibility(shelves), shelves * 2)
    }

    pub fn verify_seed(
        &mut self,
        seed: i32,
        (shelves, slot1, slot2, slot3): (i32, i32, i32, i32),
    ) -> bool {
        self.set_seed(seed as i64);
        self.levels_slot1(shelves) == slot1
            && self.levels_slot2(shelves) == slot2
            && self.levels_slot3(shelves) == slot3
    }
}
