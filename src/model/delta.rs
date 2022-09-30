#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

/// literals are attached to the chunk, where these will be placed in original file before that chunk
///
///


use std::collections::BTreeMap;

use super::hash;
use crate::fio::FileIO;
use super::chunk::Chunk;
use super::signature::Signature;
use crate::hashing::adler::Adler32;


#[derive(Debug, PartialEq)]
pub struct DiffingDelta<'local> {
    pub sign: &'local Signature,
    pub list: BTreeMap<usize, Chunk>,

}

impl<'local> DiffingDelta<'local> {
    pub fn new(sign: &'local Signature) -> Self {
        Self {
            sign,
            list: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, pos: usize, chunk: Chunk) {
        self.list.insert(pos, chunk);
    }

    pub fn get(&self, pos: usize) -> Option<&Chunk> {
        self.list.get(&pos)
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn has(&self, pos: usize) -> bool {
        self.list.contains_key(&pos)
    }

    pub fn file_to_delta_list(
        &mut self,
        path: &str,
        c_size: usize,
        file1_size: usize
    ) -> Option<()> {

        // to ne implemented..
        //...
        let bytes = FileIO::read_file_to_bytes(path);
        if bytes.is_none() { return None; }

        let bytes = bytes.unwrap();
        let len = bytes.len();
        let mut adler = Adler32::new();
        let mut literals = Vec::<u8>::new();
        let mut last_valid_pos: Option<usize> = None;
        for (i, byte) in bytes.iter().enumerate() {
            adler.roll_in(*byte);
            
            if adler.window.len() < c_size {
                // println!("continue");
                continue;
            }
            // println!("window {:?}", String::from_utf8_lossy(&adler.window[..]));
            // means no match on prev iteration
            if adler.window.len() > c_size {
                // println!("rolling on: {}", i);
                adler.roll_out();
                literals.push(adler.rolled_out_byte);
            }

            let pos = self.sign.try_get_position_of(&adler);
            // if we have match
            if pos.is_some() {
                last_valid_pos = pos;
                self.handle_match(
                    pos.unwrap(),
                    c_size,
                    &mut adler,
                    &mut literals
                );    
            }
        }
        // fill chunks which are in signature list
        // but not in delta list
        // indicate same by is_missing flag
        let last_i = self.fill_missing_chunks_if_any(c_size);
        //if its semi-filled last chunk 
        self.process_after_math(
            c_size, 
            last_valid_pos, 
            last_i, 
            file1_size, 
            &mut literals, 
            &mut adler
        );
        return Some(());
    }

    fn handle_match(
        &mut self,
        pos: usize, 
        c_size: usize,
        adler: &mut Adler32,
        literals: &mut Vec<u8> 
    ) {
        let chunk = Chunk::new(
            pos,
            c_size,
            literals.clone(),
            false,
        );
        
        self.add(pos, chunk);
        if literals.len() > 0 {
            literals.drain(..);
        }
        adler.reset();
    }

    fn fill_missing_chunks_if_any(&mut self, c_size: usize) -> usize {
        let mut i = 0;
        loop {
            if i == self.sign.list.len() {
                break;
            }
            if !self.has(i) {
                self.add(i, Chunk::new(
                    i,
                    c_size,
                    [].to_vec(),
                    true,
                ))
            }
            i += 1;
        }
        i
    }

    fn process_after_math(
        &mut self, 
        c_size: usize,
        last_valid_pos: Option<usize>,
        last_i: usize,
        len: usize,
        literals: &mut Vec<u8>,
        adler: &mut Adler32,

    ) {
        if literals.len() > 0 && adler.window.len() >= c_size {
            literals.append(&mut adler.window);
            println!("missed: {:?}", String::from_utf8_lossy(&literals[..]));
            if last_valid_pos.is_some() {
                let pos = last_valid_pos.unwrap();
                let mut chunk = self
                .list
                .get_mut(&(pos + 1));

                if chunk.is_some() {
                    // there is a chunk next to the chunk at valid pos
                    // but that obv be missing, lets add literals to that only!
                    chunk.unwrap().missing_bytes.append(literals);
                } else { // there is no chunk next to the chunk at valid position
                    // so create a new chunk and flag it not_missing with literals
                    // to be appended
                    self.add(pos+1, Chunk{
                        start_idx: pos + 1,
                        last_idx: pos + c_size,
                        missing_bytes: literals.to_vec(),
                        is_missing: false,
                    });
                }
                
            } else { // this case might not be possible
                // if there is no chunk at all
                // add our own chunk
                self.add(0, Chunk::new(
                    0,
                    c_size,
                    literals.to_vec(),
                    false,                    
                ))
            }
        }
        println!("valid pos: {:?}, size: {}", last_valid_pos, len);
        let mut res = self.list.get_mut(&last_valid_pos.unwrap());
        if res.is_some() {
            let ch = res.unwrap(); 
            if ch.last_idx >= len {
                ch.last_idx = len - 1;
            }
        }
    } 
}


#[cfg(test)]
mod delta_test {
    use super::*;
    use super::super::chunk::Chunk;
    use crate::constants::{Instruction};

    const chunk: &[u8] = &"chunk".as_bytes();

    #[test]
    fn ut_createInstance_works() {
        let mut sign = Signature::new();
        sign.add(chunk);
        
        let delta = DiffingDelta::new(&sign);
        
        assert_eq!(delta.list.len(), 0);
        assert_eq!(delta.list.get(0), None);
        
        assert_eq!(delta.sign.len(), 1);
        assert_ne!(delta.sign.get(0), None);
    }

    #[test]
    fn ut_add_get_works() {
        let mut sign = Signature::new();
        sign.add(chunk);
        
        let mut delta = DiffingDelta::new(&sign);
        
        assert_eq!(delta.list.len(), 0);
        assert_eq!(delta.list.get(0), None);
        
        assert_eq!(delta.sign.len(), 1);
        assert_ne!(delta.sign.get(0), None);

        // ------------- add -------------------
        let mut ch = Chunk::new(
            0,
            chunk.len(),
            [].to_owned(), 
            false
            // Instruction::NOP
        );

        delta.add(ch.clone());
        assert_eq!(delta.len(), 1);
        // -------------- get ------------------
        let _ch = delta.get(0);
        
        assert_ne!(_ch, None);
        // ch.is_removed = true;
        assert_eq!(_ch.unwrap(), &ch);
        
    }
}