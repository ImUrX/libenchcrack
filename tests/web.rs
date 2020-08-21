//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate libenchcrack;
extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use libenchcrack::Cracker;
use libenchcrack::utils::{Rand, Timer};
use std::num::Wrapping;
use std::panic;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn rng_seed() {
    let mut rng = Rand::new();
    rng.set_seed(0);
    assert_eq!(rng.seed, 25214903917);
    rng.set_seed(150123);
    assert_eq!(rng.seed, 25215020038);
    rng.set_seed(-500);
    assert_eq!(rng.seed, 281449761806433);
}

#[wasm_bindgen_test]
fn rng_next() {
    let mut rng = Rand::new();
    rng.set_seed(0);
    assert_eq!(rng.next(), 1569741360);
    rng.set_seed(150123);
    assert_eq!(rng.next(), 286134746);
    rng.set_seed(-500);
    assert_eq!(rng.next(), 518875706);
}

#[wasm_bindgen_test]
fn rng_next_int() {
    let mut rng = Rand::new();
    rng.set_seed(0);
    assert_eq!(rng.next_int(Wrapping(8)), 5);
    assert_eq!(rng.next_int(Wrapping(8)), 6);
    rng.set_seed(1949457528);
    assert_eq!(rng.next_int(Wrapping(8)), 3);
    assert_eq!(rng.next_int(Wrapping(5)), 0);
    rng.set_seed(-500);
    assert_eq!(rng.next_int(Wrapping(15)), 11);
}

#[wasm_bindgen_test]
fn slots() {
    let mut rng = Rand::new();
    rng.set_seed(1949457528);
    assert_eq!(rng.levels_slot1(4), 2);
    assert_eq!(rng.levels_slot2(4), 7);
    assert_eq!(rng.levels_slot3(4), 8);
    
    rng.set_seed(1949457528);
    assert_eq!(rng.levels_slot1(15), 6);
    assert_eq!(rng.levels_slot2(15), 9);
    assert_eq!(rng.levels_slot3(15), 30);
}

#[wasm_bindgen_test]
fn cracking() {
    let mut cracker = Cracker::new(0, 1);

    {
        let _timer = Timer::new("first input");
        cracker.first_input(4, 2, 7, 8, 15, 6, 9, 30);
    }
    assert!(cracker.contains(1949457528));
}
