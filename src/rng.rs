use rltk::prelude::*;
use std::sync::Mutex;

lazy_static! {
    static ref RNG: Mutex<RandomNumberGenerator> = Mutex::new(RandomNumberGenerator::new());
}

pub fn reseed(seed: u64) {
    *RNG.lock().unwrap() = RandomNumberGenerator::seeded(seed);
}

pub fn roll_dice(n: i32, die_type: i32) -> i32 {
    RNG.lock().unwrap().roll_dice(n, die_type)
}

/// <min, max> INCLUSIVE on both sides
pub fn range(min: i32, max: i32) -> i32 {
    RNG.lock().unwrap().range(min, max + 1)
}

pub fn rand_bool() -> bool {
    roll_dice(1, 2) == 1
}

/// random from 0 to 100
pub fn random_perc() -> i32 {
    range(0, 100)
}

pub fn test_perc(chance: usize) -> bool {
    let rand = random_perc() as usize;
    rand <= chance
}
