#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::constants::{Val};

#[derive(Debug)]
pub struct Change {
    // is add new change before or after a chunk
    // indicated by add_or_del_idx
    pub before: Option<bool>,
    // incase its chunk to be deleted
    // in original file this will be true
    pub del_chunk: bool,
    // this is treated as per del_chunk!
    pub add_or_del_idx: usize,
    // when new changes to add
    // this will be Some
    pub content: Option<Vec<u8>>,
}

impl PartialEq for Change {
    fn eq(&self, other: &Self) -> bool {
        self.before == other.before &&
        self.content == other.content &&
        self.del_chunk == other.del_chunk &&
        self.add_or_del_idx == other.add_or_del_idx 
    }
}

impl Clone for Change {
    fn clone(&self) -> Self {
        Self {
            before: self.before.clone(),
            content: self.content.clone(),
            del_chunk: self.del_chunk.clone(),
            add_or_del_idx: self.add_or_del_idx.clone(),
        }
    }
}

impl Change {
    pub fn new(
        before: Option<bool>,
        del_chunk: bool,
        // c_size is None if del_chunk is true and 
        // last_or_cur_match_idx is 0
        // is Some if del_chunk is false and 
        // last_or_cur_match_idx is != 0
        c_size: Option<usize>, 
        content: Option<Vec<u8>>,
        // if del_chunk true, del index else last match index
        // meaning of 0 depends on del_chunk!
        last_or_cur_match_idx: usize
    ) -> Self {
        
        if del_chunk {
            // means a chunk is to be flagged removed!
            Self {
                before,
                del_chunk,
                content: None,
                add_or_del_idx: last_or_cur_match_idx * c_size.unwrap(),
            }
        } else { 
            // means new changes to add!
            let mut add_idx = last_or_cur_match_idx;
            if c_size.is_some() {
                add_idx = last_or_cur_match_idx * c_size.unwrap();
            }
            Self {
                before,
                content,
                del_chunk,
                add_or_del_idx: add_idx,
            }
        }
    }

}


#[cfg(test)]
mod change_test {
    use super::*;

    #[test]
    fn ut_createNewChange_works() {
        let chg = Change::new(
            Some(true),
            false, 
            Some(Val::TEST_C_SIZE), 
            Some(Vec::new()), 
            1
        );
        assert_eq!(chg.del_chunk, false);
        assert_eq!(chg.add_or_del_idx, Val::TEST_C_SIZE);
        assert_eq!(chg.content.unwrap(), []);
    }
}