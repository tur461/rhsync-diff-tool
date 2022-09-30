#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use super::hash::Hash;
use crate::fio::FileIO;
use std::collections::BTreeMap;
use crate::hashing::adler::Adler32;
use crate::hashing::x2hash::X2Hash64;

#[derive(Debug, PartialEq)]
pub struct Signature {
    pub list: Vec<Hash>,
    // used to prevent duplicate chunks
    // this happens when chunk chars are same 
    // for multiple chunks in original file 
    pub traced: BTreeMap<usize, bool>,
}

impl Signature {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            traced: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, bytes: &[u8]) {
        self.list.push(
            // save as hash object
            Hash::new(bytes)
        );
    }

    pub fn get(&self, index: usize) -> Option<&Hash> {
        self.list.get(index)
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn try_get_position_of(&mut self, adler: &Adler32) -> Option<usize> {
        for (i, hash) in self.list.iter().enumerate() {
            if hash.L1 == adler.sum32() {
                // L1 hash matches;
                if 
                    hash.L2 == X2Hash64::sum64(&adler.window[..]) &&
                    // check if its a duplicate chunk already matched! 
                    !self.traced.contains_key(&i) {
                    // L2 hash matches;
                    self.traced.insert(i, true);
                    return Some(i);
                }
            }
        }
        return None;
    } 

    // convert file bytes to chunks and then list of hash objects
    pub fn file_to_sign_list(
        &mut self,
        path: &str,
        c_size: usize,
        f_size: usize
    ) -> Option<()>{
        // get list of chunks from file bytes (read internally)
        let chunk_list = FileIO::read_file_to_chunk_list(
            path, 
            c_size, 
            f_size
        );
        if chunk_list.is_none() { return None; }
        
        let chunk_list = chunk_list.unwrap();

        for chunk in chunk_list.into_iter() {
            // this adds chunk as hash object internally
            self.add(&chunk[..]);
        }
        Some(())
    }

}

#[cfg(test)]
mod sign_test {
    use super::*;
    
    const chunk: &[u8] = &"chunk".as_bytes();

    #[test]
    fn ut_createInstance_works() {
        let sign = Signature::new();
        assert_eq!(sign.list.len(), 0);
    }

    #[test]
    fn ut_add_get_works() {    
        let mut sign = Signature::new();
        // ------------- add -------------------
        sign.add(chunk);
        assert_eq!(sign.list.len(), 1);
        // -------------- get ------------------
        let hash = sign.get(0).unwrap();
        
        assert_eq!(hash.L1, 104464922);
        assert_eq!(hash.L2, 8438847523455501592);

        assert_eq!(sign.get(1), None);
    }
}

