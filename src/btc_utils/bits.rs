
// https://github.com/maciejhirsz/tiny-bip39/blob/master/src/util.rs
pub trait Bits {
    const SIZE: usize;

    fn bits(self) -> u32;
}

impl Bits for u8 {
    const SIZE: usize = 8;

    fn bits(self) -> u32 {
        self as u32
    }
}

impl<'a> Bits for &'a u8 {
    const SIZE: usize = 8;

    fn bits(self) -> u32 {
        *self as u32
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Bits11(u16);

impl Bits for Bits11 {
    const SIZE: usize = 11;

    fn bits(self) -> u32 {
        self.0 as u32
    }
}

impl From<u16> for Bits11 {
    fn from(val: u16) -> Self {
        Bits11(val)
    }
}

impl From<Bits11> for u16 {
    fn from(val: Bits11) -> Self {
        val.0
    }
}

pub fn to_binary(input: String) -> String {
    let mut input_in_binary = "".to_string();
    
    // Call into_bytes() which returns a Vec<u8>, and iterate accordingly
    for character in input.clone().into_bytes() {
        input_in_binary += &format!("0{:b} ", character);
    }

    input_in_binary.trim_end().to_string()
}

pub fn checksum(source: u8, bits: u8) -> u8 {
    debug_assert!(bits <= 8, "Can operate on 8-bit integers only");

    source >> (8 - bits)
}