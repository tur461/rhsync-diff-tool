#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(arithmetic_overflow)]

use crate::constants::Val;

// largest 16-bit prime number
const BASE: u32 = 65521;
// NMAX is the largest n such that 255n(n+1)/2 + (n+1)(BASE-1) <= 2^32-1
const NMAX: usize = 5552;

#[inline(always)]
fn do1(s1: &mut u32, s2: &mut u32, bytes: &[u8]) {
    *s1 += bytes[0] as u32;
    *s2 += *s1;
}

#[inline(always)]
fn do2(s1: &mut u32, s2: &mut u32, bytes: &[u8]) {
    do1(s1, s2, &bytes[0..1]);
    do1(s1, s2, &bytes[1..2]);
}

#[inline(always)]
fn do4(s1: &mut u32, s2: &mut u32, bytes: &[u8]) {
    do2(s1, s2, &bytes[0..2]);
    do2(s1, s2, &bytes[2..4]);
}

#[inline(always)]
fn do8(s1: &mut u32, s2: &mut u32, bytes: &[u8]) {
    do4(s1, s2, &bytes[0..4]);
    do4(s1, s2, &bytes[4..8]);
}

#[inline(always)]
fn do16(s1: &mut u32, s2: &mut u32, bytes: &[u8]) {
    do8(s1, s2, &bytes[0..8]);
    do8(s1, s2, &bytes[8..16]);
}

#[derive(Debug, PartialEq)]
pub struct Adler32 {
    pub s1: u32, 
    pub s2: u32,
    pub count: usize,
    pub window: Vec<u8>,
    pub rolled_out_byte: u8,
}

impl Adler32 {
    /// Creates an empty Adler32 ctx (with hash 1).
    pub fn new() -> Self {
        Self {
            s1: 1u32  & 0xFFFF,
            s2: 1u32 >> 16,
            count: 0,
            rolled_out_byte: 0,
            window: Vec::new(),
        }
    }

    pub fn roll_in(&mut self, byte: u8) -> &mut Self {
        self.s1 = (self.s1 + byte as u32) % BASE;
        self.s2 = (self.s1 + self.s2) % BASE;
        
        self.window.push(byte);
        self.count += 1;
        self
    }

    /// Convenience function initializing a context from the hash of a buffer.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut adler32 = Adler32::new();
        adler32.write_bytes(bytes);
        adler32
    }
    
    pub fn roll_out(& mut self) -> &mut Self {
        let len = self.window.len() as u32;
        if len == 0 {
            self.count = 0;
            return self;
        }
        self.rolled_out_byte = self.window.remove(0);

        let rolled_out_byte = self.rolled_out_byte as u32;

        self.s1 = (self.s1 + BASE - rolled_out_byte) % BASE;
        self.s2 = ((self.s2 + BASE - 1).wrapping_add(BASE.wrapping_sub(len).wrapping_mul(rolled_out_byte))) % BASE;
        
        self.count -= 1;
        self
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> &mut Self {
        let len = bytes.len();
         // in case user likes doing a byte at a time, keep it fas
        if len == 1 {
            self.roll_in(bytes[0]);
            self.count += 1;
            return self;
        }
        // in case short lengths are provided, keep it somewhat fast
        if len < 16 {
            for byte in bytes.iter().take(len) {
                self.s1 += *byte as u32;
                self.s2 += self.s1;
                self.count += 1;
            }
            
            if self.s1 >= BASE {
                self.s1 -= BASE;
            }

            self.s2 %= BASE;
            
            return self;
        }

        let mut pos = 0;

        // do length NMAX blocks -- requires just one modulo operation;
        while pos + NMAX <= len {
            let end = pos + NMAX;
            while pos < end {
                // 16 sums unrolled
                do16(&mut self.s1, &mut self.s2, &bytes[pos..pos + 16]);
                pos += 16;
                self.count += 16;
            }
            self.s1 %= BASE;
            self.s2 %= BASE;
        }

        // do remaining bytes (less than NMAX, still just one modulo)
        if pos < len {
            // avoid modulos if none remaining
            while len - pos >= 16 {
                do16(&mut self.s1, &mut self.s2, &bytes[pos..pos + 16]);
                pos += 16;
                self.count += 16;
            }
            while len - pos > 0 {
                self.s1 += bytes[pos] as u32;
                self.s2 += self.s1;
                pos += 1;
                self.count += 1;
            }
            self.s1 %= BASE;
            self.s2 %= BASE;
        }
        self
    }

    pub fn sum32(&self) -> u32 {
        self.s2  << 16 | self.s1
    }

    pub fn reset(&mut self) {
        self.s1 = 1u32  & 0xFFFF;
        self.s2 = 1u32 >> 16;
        self.count = 0;
        self.window.drain(..);
    }
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn ut_testVectors_works() {
        fn do_test(v: u32, bytes: &[u8]) {
            let mut adler = Adler32::new();
            adler.write_bytes(&bytes);
            assert_eq!(adler.sum32(), v);
        }

        do_test(0x00000001, b"");
        do_test(0x00620062, b"a");
        do_test(0x024d0127, b"abc");
        do_test(0x29750586, b"message digest");
        do_test(0x90860b20, b"abcdefghijklmnopqrstuvwxyz");
        do_test(
            0x8adb150c,
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        );
        do_test(
            0x97b61069,
            b"12345678901234567890123456789012345678901234567890123456789012345678901234567890",
        );
        do_test(0xD6251498, &[255; 64000]);
    }


    #[test]
    fn ut_rolling_works() {
        const a: &[u8] = &"the quick brown fox jumped over the lazy dog!".as_bytes();
        const b: &[u8] = &"@the Quick brown fox jumped over the lazy dog!".as_bytes();

        let mut adler1 = Adler32::new();
        let mut adler2 = Adler32::new();
        
        let hash1 = adler1.write_bytes(&a[..4]).sum32();
        assert_eq!(adler1.count, 4);

        for c in &b[..4] {
            adler2.roll_in(*c);
        }

        assert_eq!(adler2.count, 4);
        assert_eq!(adler2.window, b[..4]);
        assert_ne!(adler1.window, adler2.window);
        // hashes must not mach
        assert_ne!(adler1.sum32(), adler2.sum32());
        // -------------- rolling --------------
        // roll window by 1 step 
        adler2.roll_out();

        let mut c = adler2.rolled_out_byte;
        
        assert_eq!(c, b[0]);
        assert_eq!(adler2.count, 3);
        assert_eq!(adler2.window, b[1..4]);

        adler2.roll_in(b[4]);

        assert_eq!(adler2.count, 4);
        assert_eq!(adler2.window, b[1..5]);
        // now hashes must mach
        assert_eq!(adler2.sum32(), adler2.sum32());
        // ----------------------------------------
        
        assert_eq!(adler2.window, b[1..5]);
        // ----------------------------------------

    }
}