
#[non_exhaustive]
pub struct Val;
pub struct SharedError;

impl Val {
    pub const ZERO_USIZE: usize = 0;
    pub const TEST_C_SIZE: usize = 4;
    pub const TEST_F_SIZE: usize = 80;
}

impl SharedError {
    pub const CHUNK_SIZE_ZERO: &'static str = "Chunk size must be non zero!";
    pub const AT_LEAST_TWO_CHUNKS: &'static str = "chunk size invalid, must have at least 2 chunks for the file";
}