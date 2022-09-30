#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::constants::{Val};
use crate::hashing::adler::Adler32;
use crate::hashing::x2hash::X2Hash64;

#[derive(Debug, PartialEq)]
pub struct Hash {
    // level 1 = adler32 rolling hash
    pub L1: u32,
    // level 2 = xx3hash 64 bit non-rolling
    pub L2: u64,
}

impl Hash {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            L2: X2Hash64::sum64(bytes),
            L1: Adler32::new().write_bytes(bytes).sum32(),
        }
    }
}


#[cfg(test)]
mod hash_test {
    use super::*;

    #[test]
    fn ut_createNewHash_works() {
        const chunk: &[u8] = &"chunk".as_bytes();
        let hash = Hash::new(chunk);
        assert_eq!(hash.L1, 104464922);
        assert_eq!(hash.L2, 8438847523455501592);
    }
}