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
            while {r = u % bound; u - r + m}.0 < 0 {
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
        if slot1 < 1 { 1 } else { slot1 }
    }

    fn levels_slot2(&mut self, shelves: i32) -> i32 {
        (self.generic_enchantibility(shelves) * 2 / 3) + 1
    }

    fn levels_slot3(&mut self, shelves: i32) -> i32 {
        cmp::max(self.generic_enchantibility(shelves), shelves * 2)
    }

    pub fn first_input(&mut self, seed: i32, (shelves, slot1, slot2, slot3): (i32, i32, i32, i32)) -> bool {
        let two_shelves = shelves * 2;
        let half_shelves = shelves / 2 + 1;
        let shelves_plus_one = shelves + 1;

        let first_early = slot1 * 3 + 2;
        let second_early = slot2 * 3 / 2;
        let second_sub_one = slot2 - 1;
        self.set_seed(seed as i64);

        let ench1r1 = self.next_int_bound(Wrapping(8)) + half_shelves;
        if ench1r1 > first_early { return false; }
        let ench1 = (ench1r1 + self.next_int_bound(Wrapping(shelves_plus_one))) * 2 / 3;
        if (ench1 < 1 && slot1 != 1) || ench1 != slot1 { return false; }

        let ench2r1 = self.next_int_bound(Wrapping(8)) + half_shelves;
        if ench2r1 > second_early { return false; }
        let ench2 = (ench2r1 + self.next_int_bound(Wrapping(shelves_plus_one))) * 2 / 3;
        if ench2 != second_sub_one { return false; }

        let ench3 = self.next_int_bound(Wrapping(8)) + half_shelves + self.next_int_bound(Wrapping(shelves_plus_one));
        if cmp::max(ench3, two_shelves) != slot3 { return false; }

        true
    }

    pub fn verify_seed(&mut self, seed: i32, (shelves, slot1, slot2, slot3): (i32, i32, i32, i32)) -> bool {
        self.set_seed(seed as i64);
        self.levels_slot1(shelves) == slot1 && self.levels_slot2(shelves) == slot2 && self.levels_slot3(shelves) == slot3
    }
}
