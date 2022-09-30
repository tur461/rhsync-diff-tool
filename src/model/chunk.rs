#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::constants::{Val, Instruction};

#[derive(Debug)]
pub struct Chunk {
    pub last_idx: usize,
    pub start_idx: usize,
    pub is_missing: bool, 
    pub missing_bytes: Vec<u8>,
    // pub instruction: Instruction,
}

impl PartialEq for Chunk {
    fn eq(&self, other: &Self) -> bool {
        self.last_idx == other.last_idx &&
        self.start_idx == other.start_idx &&
        self.is_missing == other.is_missing &&
        self.missing_bytes == other.missing_bytes
    }
}

impl Clone for Chunk {
    fn clone(&self) -> Self {
        Self {
            last_idx: self.last_idx.clone(),
            start_idx: self.start_idx.clone(),
            is_missing: self.is_missing.clone(),
            missing_bytes: self.missing_bytes.clone(),
        }
    }
}

impl Chunk {
    pub fn new(
        cur_idx: usize, 
        c_size: usize, 
        bytes: Vec<u8>, 
        is_missing: bool,
        // instruction: Instruction,
    ) -> Self {
        Self {
            is_missing,
            // instruction,
            missing_bytes: bytes,
            start_idx: (cur_idx * c_size), 
            last_idx: (cur_idx * c_size) + c_size
        }
    }

}


#[cfg(test)]
mod chunk_test {
    use super::*;

    #[test]
    fn ut_createNewChunk_works() {
        let ck = Chunk::new(
            Val::TEST_INDEX, 
            Val::TEST_C_SIZE, 
            Vec::new(), 
            false 
            // Instruction::NOP
        );
        assert_eq!(ck.is_missing, false);
        assert_eq!(ck.missing_bytes, []);
        // assert_eq!(ck.instruction, Instruction::NOP);
        assert_eq!(ck.start_idx, Val::TEST_INDEX * Val::TEST_C_SIZE);
        assert_eq!(ck.last_idx, (Val::TEST_INDEX * Val::TEST_C_SIZE) + Val::TEST_C_SIZE);
    }
}