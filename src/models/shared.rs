#![allow(non_snake_case)]

use crate::utils::Utility;
use crate::constants::{ SharedError, Val};

pub struct Shared {
    chunk_size: usize,
    orig_file_size: usize,
    modified_file_size: usize,
}

impl Shared {
    pub fn new(c_size: usize) -> Self {
        Self::panic_if_invalid_chunk_size(c_size, None);
        Self {
            chunk_size: c_size,
            orig_file_size: 0,
            modified_file_size: 0,
        }
    }

    pub fn panic_if_invalid_chunk_size(c_size: usize, f_size: Option<usize>) {
        if let Some(fs) = f_size {
            if c_size >= fs {
                Utility::panic(SharedError::AT_LEAST_TWO_CHUNKS);
            }
        }

        if c_size == Val::ZERO_USIZE {    
            Utility::panic(SharedError::CHUNK_SIZE_ZERO);
        }
        
    }
    
    pub fn get_chunk_size(&self) -> usize {
        self.chunk_size
    }
    pub fn get_file_size_original(&self) -> usize {
        self.orig_file_size
    }
    pub fn get_file_size_modified(&self) -> usize {
        self.modified_file_size
    }
    
    pub fn update_chunk_size(&mut self, c_size: usize) {
        Self::panic_if_invalid_chunk_size(c_size, None);
        self.chunk_size = c_size;    
    }

    pub fn set_file_size_original(&mut self, f_size: usize) {
        Self::panic_if_invalid_chunk_size(self.chunk_size, Some(f_size));
        self.orig_file_size = f_size;
    }

    pub fn set_file_size_modified(&mut self, f_size: usize) {
        Self::panic_if_invalid_chunk_size(self.chunk_size, Some(f_size));
        self.modified_file_size = f_size;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn ut_createInstanceOfSharedModelWithZeroChunkSizeFails() {
        let sm = Shared::new(Val::ZERO_USIZE);
    }

    #[test]
    fn ut_createInstanceOfSharedModelWorks() {
        let sm = Shared::new(Val::TEST_C_SIZE);
        assert_eq!(sm.get_chunk_size(), Val::TEST_C_SIZE);
        assert_eq!(sm.get_file_size_original(), Val::ZERO_USIZE);
        assert_eq!(sm.get_file_size_modified(), Val::ZERO_USIZE);
    }
    
    #[test]
    fn ut_getChunkSize_works() {
        let sm = Shared::new(Val::TEST_C_SIZE);
        assert!(sm.get_chunk_size() == Val::TEST_C_SIZE);
    }
    
    #[test]
    fn ut_getOriginalFileSize_works() {
        let sm = Shared::new(Val::TEST_C_SIZE);
        assert!(sm.get_file_size_original() == Val::ZERO_USIZE);
    }
    
    #[test]
    fn ut_getModifiedFileSize_works() {
        let sm = Shared::new(Val::TEST_C_SIZE);
        assert!(sm.get_file_size_modified() == Val::ZERO_USIZE);
    }

    #[test]
    fn ut_updateChunkSize_works() {
        let mut sm = Shared::new(Val::TEST_C_SIZE);
        sm.update_chunk_size(Val::TEST_C_SIZE + 1);
        assert!(sm.get_chunk_size() == Val::TEST_C_SIZE + 1);
    }

    #[test]
    #[should_panic]
    fn ut_updateChunkSizeWithZero_fails() {
        let mut sm = Shared::new(Val::TEST_C_SIZE);
        sm.update_chunk_size(Val::ZERO_USIZE);
    }

    #[test]
    fn ut_setOriginalFileSize_works() {
        let mut sm = Shared::new(Val::TEST_C_SIZE);
        sm.set_file_size_original(Val::TEST_F_SIZE);
        assert!(sm.get_file_size_original() == Val::TEST_F_SIZE);
    }

    #[test]
    fn ut_setModifiedFileSize_works() {
        let mut sm = Shared::new(Val::TEST_C_SIZE);
        sm.set_file_size_modified(Val::TEST_F_SIZE);
        assert!(sm.get_file_size_modified() == Val::TEST_F_SIZE);
    }

    #[test]
    #[should_panic]
    fn ut_setOriginalFileSizeWithInvalidChunkSize_fails() {
        let mut sm = Shared::new(Val::TEST_F_SIZE);
        sm.set_file_size_original(Val::TEST_F_SIZE);
    }

    #[test]
    #[should_panic]
    fn ut_setModifiedFileSizeWithInvalidChunkSize_fails() {
        let mut sm = Shared::new(Val::TEST_F_SIZE);
        sm.set_file_size_modified(Val::TEST_F_SIZE);
    }
}
