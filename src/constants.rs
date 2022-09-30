#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]


#[non_exhaustive]
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    ADD,
    DEL,
    MOV,
    NOP,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ErrKind {
    FILE_OPEN,

}


#[non_exhaustive]
#[derive(Debug)]
pub struct Val;

#[derive(Debug)]
pub struct SharedError;

impl Val {
    pub const ZERO_U8: u8 = 0;
    pub const ZERO_U16: u16 = 0;
    pub const ZERO_U32: u32 = 0;
    pub const ZERO_U64: u64 = 0;
    pub const ONE_USIZE: usize = 1;
    pub const ZERO_USIZE: usize = 0;
    pub const TEST_INDEX: usize = 1;
    pub const TEST_C_SIZE: usize = 4;
    pub const TEST_F_SIZE: usize = 80;
    pub const DEFAULT_C_SIZE: usize = 4;
    pub const MIN_NUM_OF_CHUNKS: usize = 2;
}

impl SharedError {
    pub const CHUNK_SIZE_ZERO: &'static str = "Chunk size must be non zero!";
    pub const AT_LEAST_TWO_CHUNKS: &'static str = "chunk size invalid, must have at least 2 chunks for the file";
}