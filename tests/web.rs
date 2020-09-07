//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate libenchcrack;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use libenchcrack::*;
use libenchcrack::manipulation::*;
use libenchcrack::utils::SimpleRandom;
use web_sys::console;
use std::num::Wrapping;
use std::panic;

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

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn rng_seed() {
    let mut rng = SimpleRandom::new();
    rng.set_seed(0);
    assert_eq!(rng.seed, 25214903917);
    rng.set_seed(150123);
    assert_eq!(rng.seed, 25215020038);
    rng.set_seed(-500);
    assert_eq!(rng.seed, 281449761806433);
}

#[wasm_bindgen_test]
fn rng_next() {
    let mut rng = SimpleRandom::new();
    rng.set_seed(0);
    assert_eq!(rng.next(), 1569741360);
    rng.set_seed(150123);
    assert_eq!(rng.next(), 286134746);
    rng.set_seed(-500);
    assert_eq!(rng.next(), 518875706);
}

#[wasm_bindgen_test]
fn rng_next_int() {
    let mut rng = SimpleRandom::new();
    rng.set_seed(0);
    assert_eq!(rng.next_int(Wrapping(8)), 5);
    assert_eq!(rng.next_int(Wrapping(8)), 6);
    rng.set_seed(1949457528);
    assert_eq!(rng.next_int(Wrapping(8)), 3);
    assert_eq!(rng.next_int(Wrapping(5)), 0);
    rng.set_seed(-500);
    assert_eq!(rng.next_int(Wrapping(15)), 11);
}

/*#[wasm_bindgen_test] 
fn testings() {
    let item = Item::NetheriteSword;
    let mut rand = java_rand::Random::new(10);
    rand.set_seed(10);
    assert!(!Version::V1_16.before(item.get_introduced_version()));
    for ench in Enchantment::get_highest_allowed_enchantments(30, item, false, Version::V1_16) {
        console_log!("{:#?}", ench);
    }
}*/

#[wasm_bindgen_test]
fn manipulator() {
    let mut man = Manipulator::new(2893231007, 2635886329, Item::NetheriteSword).expect("Wrong seeds");
    let hex = vec!(0x2e, 0x3d, 0xf9, 0x6e, 0x1c, 0x9d);
    assert_eq!(hex.len(), 6); // 6 bytes
    assert_eq!(man.player_seed().to_vec(), hex);
    man.want(EnchantmentInstance::new(Enchantment::BaneOfArthropods, 5));
    man.want(EnchantmentInstance::new(Enchantment::FireAspect, 2));
    man.not_want(EnchantmentInstance::new(Enchantment::Knockback, 1));
    let v = man.simulate(15, 999, Version::V1_16).expect("Simulation Failed").to_vec();
    assert_eq!(v, vec!(57, 15, 3));
}

/*#[wasm_bindgen_test]
fn cracking() {
    let mut cracker = Cracker::new(0, 1);

    {
        let _timer = Timer::new("first input");
        cracker.first_input(15, 5, 20, 30, 12, 5, 10, 24);
    }
    let first_amount = cracker.possible_seeds();
    assert!(cracker.contains(-329083225));

    {
        let _timer = Timer::new("second input");
        cracker.add_input(8, 2, 10, 16);
    }
    assert_ne!(cracker.possible_seeds(), first_amount);
    assert!(cracker.contains(-329083225));

    {
        let _timer = Timer::new("third input");
        cracker.add_input(6, 3, 9, 12);
    }
    assert!(cracker.contains(-329083225));

    {
        let _timer = Timer::new("fourth input");
        cracker.add_input(4, 1, 9, 8);
    }
    assert!(cracker.contains(-329083225));

    {
        let _timer = Timer::new("fifth input");
        cracker.add_input(3, 1, 9, 6);
    }
    assert!(cracker.contains(-329083225));

    {
        let _timer = Timer::new("sixth input");
        cracker.add_input(7, 2, 13, 14);
    }
    assert!(cracker.contains(-329083225));

    {
        let _timer = Timer::new("seventh input");
        cracker.add_input(14, 6, 12, 28);
    }
    assert!(cracker.contains(-329083225));

    assert_eq!(cracker.possible_seeds(), 1)
}*/
