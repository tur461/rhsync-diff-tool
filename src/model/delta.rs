#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

/*
    During patching, this program's output works the following way:
    
    1. first all the Changes (Change object) with del_chunk true, will be applied where 
        add_or_del_idx indicates index of the chunk in original file.
    2. After 1st step, apply Changes with del_chunk = false before/after the chunk whose index 
        is indicated by add_or_del_idx . before or after is indicated by before flag

    Note:
        for before true apply those changes first, then apply those for which before is false  
*/

use super::hash;
use crate::fio::FileIO;
use super::changes::Change;
use super::signature::Signature;
use crate::hashing::adler::Adler32;


#[derive(Debug, PartialEq)]
pub struct DiffingDelta<'local> {
    pub sign: &'local mut Signature,
    pub list: Vec<Change>,

}

impl<'local> DiffingDelta<'local> {
    pub fn new(sign: &'local mut Signature) -> Self {
        Self {
            sign,
            list: Vec::new(),
        }
    }

    pub fn add(&mut self, change: Change) {
        self.list.push(change);
    }

    pub fn get(&self, index: usize) -> Option<&Change> {
        self.list.get(index)
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn file_to_delta_list(
        &mut self,
        path: &str,
        c_size: usize
    ) -> Option<()> {
        // get all bytes from file
        let bytes = FileIO::read_file_to_bytes(path);
        if bytes.is_none() { return None; }

        let bytes = bytes.unwrap();
        let len = bytes.len();
        let mut adler = Adler32::new();
        let mut last_match_idx = 0usize;
        let mut literals = Vec::<u8>::new();
        let mut matched_chunks = Vec::<usize>::new();
        for (i, byte) in bytes.iter().enumerate() {
            adler.roll_in(*byte);
            
            if adler.window.len() < c_size && i < len-1 {
                continue;
            }
            // means no match on prev iteration
            if adler.window.len() > c_size {
                // this makes window sliding/rolling
                adler.roll_out();
                // save rolled out bytes into literals
                // so it could be added to change object later-on
                literals.push(adler.rolled_out_byte);
            }

            // try getting a match
            let idx = self.sign.try_get_position_of(&adler);
            // if we have match
            if idx.is_some() {
                last_match_idx = idx.unwrap();
                // save cur match idx for later use
                matched_chunks.push(idx.unwrap());
                // if we have some unsaved changes
                if literals.len() > 0 {
                    let mut temp_cz = Some(c_size);
                    if last_match_idx == 0 {
                        temp_cz = None;
                    }
                    // save change into delta list
                    self.handle_new_change(
                        Some(true),
                        last_match_idx,
                        temp_cz,
                        literals.clone()
                    );
                    // reset literals
                    literals.drain(..);
                }
                // reset adler for finding next chunk
                adler.reset();
            }
        }
        // fill chunks which are in signature list
        // but not in delta list
        // indicate same by del_chunk flag
        self.fill_missing_chunks_if_any(&matched_chunks, c_size);
        
        // corner-case: (where above loop fails)
        // we have missed some new changes in main loop
        // because it ended or last chunk didn't match
        // and remained in adler.window
        // check if there are chars in window which didn't got matched
        // and loop ended
        // if nothing in there then check whats in literals
        if adler.window.len() > 0 || literals.len() > 0 {
            if adler.window.len() > 0 {
                // put window chars into literals
                literals.append(&mut adler.window)
            }
            // just handling indexing
            if matched_chunks.len() > 0 && last_match_idx == 0 {
                last_match_idx += c_size;
            } else {
                last_match_idx = (last_match_idx * c_size) + c_size;
            }
            // finally add the literals as new change
            // into delta list but with 'before' flag set false
            // means add these changes 'after' last chunk matched!
            self.handle_new_change(
                Some(false),
                last_match_idx,
                None,
                literals
            )
        }

        return Some(());
    }

    fn handle_new_change(
        &mut self,
        before: Option<bool>,
        last_match_idx: usize, 
        c_size: Option<usize>,
        literals: Vec<u8> 
    ) {
        // create a Change object
        let change = Change::new(
            before,
            false,
            c_size,
            Some(literals.clone()),
            last_match_idx
        );
        // finally save into the delta list!!
        self.add(change);
    }

    fn fill_missing_chunks_if_any(
        &mut self,
        matched_chunks: &Vec<usize>,
        cz: usize
    ) {
        let mut i = 0;
        let mut j = 0;
        let len1 = matched_chunks.len();
        let len2 = self.sign.list.len();
        loop {
            // j depends on sign list
            // we will terminate loop on that only
            // as matched chunks may be less than that
            if j == len2 {
                break;
            }
            
            if  i < len1 && matched_chunks[i] == j{
                // we hit a matched block in signatures
                i += 1;
            } else {
                self.add(Change::new(
                    None,
                    true, // this flag make chunk removed
                    Some(cz),
                    None,
                    j,
                ))
            }
            j += 1;
        }
    }
}


#[cfg(test)]
mod delta_test {
    use super::*;

    const chunk: &[u8] = &"chunk".as_bytes();

    #[test]
    fn ut_createInstance_works() {
        let mut sign = Signature::new();
        sign.add(chunk);
        
        let delta = DiffingDelta::new(&mut sign);
        
        assert_eq!(delta.list.len(), 0);
        assert_eq!(delta.list.get(0), None);
        
        assert_eq!(delta.sign.len(), 1);
        assert_ne!(delta.sign.get(0), None);
    }

    #[test]
    fn ut_add_get_works() {
        let mut sign = Signature::new();
        sign.add(chunk);
        
        let mut delta = DiffingDelta::new(&mut sign);
        
        assert_eq!(delta.list.len(), 0);
        assert_eq!(delta.list.get(0), None) ;
        
        assert_eq!(delta.sign.len(), 1);
        assert_ne!(delta.sign.get(0), None);

        // ------------- add -------------------
        let mut ch = Change::new(
            Some(true),
            false,
            None,
            Some(chunk[0..3].to_owned()), 
            0
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