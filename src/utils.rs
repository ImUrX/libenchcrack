#[cfg(target_feature="simd128")]
use std::arch::wasm32::*;
use std::cmp;
use std::num::Wrapping;

const MULT: i64 = 0x5DEECE66D;
#[cfg(target_feature="simd128")]
#[allow(non_upper_case_globals)]
const MULTx2: v128 = i64x2_splat(MULT);

const MASK: i64 = (1 << 48) - 1;
#[cfg(target_feature="simd128")]
#[allow(non_upper_case_globals)]
const MASKx2: v128 = i64x2_splat(MASK);

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

#[cfg(target_feature="simd128")]
pub struct SIMDSimpleRandom {
    pub seeds: [v128; 2] //supposedly 2 i64x2
}

#[cfg(target_feature="simd128")]
impl SIMDSimpleRandom {
    pub fn set_seed(&mut self, seeds: [v128; 2]) {
        //self.seed = (seed ^ MULT) & MASK;
        self.seeds = seeds.map(|seed| v128_and(v128_xor(seed, MULTx2), MASKx2))
    }

    pub fn next_int(&mut self) -> v128 { //supposedly i32x4
        //self.seed = (Wrapping(self.seed) * Wrapping(MULT) + Wrapping(0xB)).0 & MASK;
        //(self.seed as u64 >> 17) as i32
        self.seeds = self.seeds.map(|seed| v128_and(i64x2_add(i64x2_mul(seed, MULTx2), i64x2_splat(0xB)), MASKx2));
        let unsigned: [v128; 2] = self.seeds.map(|seed| u64x2_shr(seed, 17));
        i32x4(i64x2_extract_lane::<0>(unsigned[0]) as i32, i64x2_extract_lane::<1>(unsigned[0]) as i32, i64x2_extract_lane::<0>(unsigned[1]) as i32, i64x2_extract_lane::<1>(unsigned[1]) as i32)
    }

    pub fn get_seed(&self, index: usize) -> i64 {
        let i = if index > 1 { 1 } else { 0 };
        match index {
            0 => i64x2_extract_lane::<0>(self.seeds[i]),
            1 => i64x2_extract_lane::<1>(self.seeds[i]),
            2 => i64x2_extract_lane::<0>(self.seeds[i]),
            3 => i64x2_extract_lane::<1>(self.seeds[i]),
            _ => panic!("The total size is 4!")
        }
    }

    pub fn replace_seed(&mut self, seed: i64, index: usize) {
        let i = if index > 1 { 1 } else { 0 };
        self.seeds[i] = match index {
            0 => i64x2_replace_lane::<0>(self.seeds[i], seed),
            1 => i64x2_replace_lane::<1>(self.seeds[i], seed),
            2 => i64x2_replace_lane::<0>(self.seeds[i], seed),
            3 => i64x2_replace_lane::<1>(self.seeds[i], seed),
            _ => panic!("The total size is 4!")
        }
    }

    pub fn next_int_indexed(&mut self, index: usize) -> i32 {
        let mut seed: i64 = self.get_seed(index);
        seed = (Wrapping(seed) * Wrapping(MULT) + Wrapping(0xB)).0 & MASK;
        self.replace_seed(seed, index);
        (seed as u64 >> 17) as i32
    }

    pub fn next_int_bound(&mut self, bound_num: i32) -> v128 { //supposedly i32x4
        /*let mut r = Wrapping(self.next_int());
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
        r.0*/
        let bound = i32x4_splat(bound_num);
        let mut r = self.next_int();
        let m_num = (Wrapping(bound_num) - Wrapping(1)).0;
        if (bound_num & m_num) == 0 {
            let high = i64x2_shr(i64x2_extmul_high_i32x4(bound, r), 31);
            let low = i64x2_shr(i64x2_extmul_low_i32x4(bound, r), 31);
            return i32x4(i64x2_extract_lane::<0>(low) as i32, i64x2_extract_lane::<1>(low) as i32, i64x2_extract_lane::<0>(high) as i32, i64x2_extract_lane::<1>(high) as i32)
        }

        let m = i32x4_splat(m_num);
        let boundf = f32x4_convert_i32x4(bound);
        let mut u = r;
        let mut flags = [false, false, false, false];
        loop {
            r = i32x4_sub(u, i32x4_mul(i32x4_trunc_sat_f32x4(f32x4_div(f32x4_convert_i32x4(u), boundf)), bound));
            let comp = i32x4_lt(i32x4_add(i32x4_sub(u, r), m), i32x4_splat(0));
            if !i32x4_all_true(comp) {
                flags[0] = i32x4_extract_lane::<0>(comp) != 0;
                flags[1] = i32x4_extract_lane::<1>(comp) != 0;
                flags[2] = i32x4_extract_lane::<2>(comp) != 0;
                flags[3] = i32x4_extract_lane::<3>(comp) != 0;
                break;
            }
            u = self.next_int();
        }

        for i in 0..4 {
            if !flags[i] { continue; }
            let mut u_num = get_i32(u, i);
            let mut res_num: i32;
            loop {
                res_num = u_num % bound_num;
                if (u_num - res_num + m_num) >= 0 { break; }
                u_num = self.next_int_indexed(i);
            }
            r = match i {
                0 => i32x4_replace_lane::<0>(r, res_num),
                1 => i32x4_replace_lane::<1>(r, res_num),
                2 => i32x4_replace_lane::<2>(r, res_num),
                3 => i32x4_replace_lane::<3>(r, res_num),
                _ => panic!("The total size is 4!")
            };
        }
        r
    }

    fn generic_enchantibility(&mut self, shelves: i32) -> v128 {
        let first = self.next_int_bound(8);
        let second = self.next_int_bound(shelves + 1);
        //first + 1 + (shelves >> 1) + second
        i32x4_add(i32x4_add(first, i32x4_splat(1)), i32x4_add(i32x4_splat(shelves >> 1), second))
    }

    /*fn levels_slot1(&mut self, shelves: i32) -> v128 {
        //let slot1 = self.generic_enchantibility(shelves) / 3;
        let slot1 = 
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
    }*/
}

#[cfg(target_feature="simd128")]
fn get_i32(vec: v128, index: usize) -> i32 {
    match index {
        0 => i32x4_extract_lane::<0>(vec),
        1 => i32x4_extract_lane::<1>(vec),
        2 => i32x4_extract_lane::<2>(vec),
        3 => i32x4_extract_lane::<3>(vec),
        _ => panic!("The total size is 4!")
    }
}
