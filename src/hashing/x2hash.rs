#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use xxh3::hash64_with_seed;

const SEED: u64 = 0;

pub struct X2Hash64;

impl X2Hash64 {
    pub fn sum64(bytes: &[u8]) -> u64 {
        hash64_with_seed(bytes, SEED)
    }
}