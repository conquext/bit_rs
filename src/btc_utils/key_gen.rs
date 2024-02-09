use rand::{thread_rng, RngCore};
use std::fmt;
use sha2::{Sha256, Digest};
extern crate hex;

use super::{language::Language, stretch_key::stretch_key};

// const SEED_LENGTH: usize = 24;

#[derive(Clone)]
pub struct Seed {
    pub bytes: Vec<u8>,
}

impl  Seed {
    /// Get the seed value as a byte slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl fmt::Debug for Seed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#X}", self)
    }
}

impl fmt::LowerHex for Seed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }

        for byte in &self.bytes {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}

impl fmt::UpperHex for Seed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }

        for byte in &self.bytes {
            write!(f, "{:02X}", byte)?;
        }

        Ok(())
    }
}

fn gen_random_bytes(byte_length: usize) -> Vec<u8> {
    let mut rng = thread_rng();
    let mut bytes = vec![0u8; byte_length];

    rng.fill_bytes(&mut bytes);

    bytes
}

pub fn sha256_first_byte(input: &[u8]) -> u8 {
    Sha256::digest(input)[0]
}

fn split_into_11_bit_segments(sequence: Vec<u8>) -> Vec<u16> {
    let mut result = Vec::new();
    let mut current_segment = 0u16;
    let mut bit_count = 0;

    for &byte in &sequence {
        current_segment = (current_segment << 8) | byte as u16;
        bit_count += 8;

        while bit_count >= 11 {
            let segment = (current_segment >> (bit_count - 11)) & 0x7FF;
            result.push(segment as u16);
            bit_count -= 11;
        }
    }

    if bit_count > 0 {
        current_segment <<= (11 - bit_count);
        result.push(current_segment);
    }

    result
}


pub fn generate_entropy() -> Vec<u8> {
    // let entropy_bits = 24 >> 8;
    // let entropy = gen_random_bytes(entropy_bits / 8);
    gen_random_bytes(32)
}

pub fn generate_mnemonic_phrase() -> String {
    let entropy = generate_entropy();

    generate_mnemonic_phrase_from_entropy(entropy)    
}

pub fn generate_mnemonic_phrase_from_entropy(entropy: Vec<u8>) -> String {
    let checksum_byte = sha256_first_byte(&entropy);

    let entropy = entropy
            .iter()
            .chain(Some(&checksum_byte)).map(|&bit| bit).collect();

    let word_list = Language::English.wordlist();

    let bits_11: Vec<&str> = split_into_11_bit_segments(entropy).into_iter().map(|bit| word_list.get_word2(bit)).collect();

    bits_11.join(" ")    
}

pub fn generate_seed_from_mnemonic (mnemonic: &str, password: &str) -> Vec<u8> {
    let salt = format!("mnemonic{}", password);
    let bytes = stretch_key(mnemonic, &salt).into();
    bytes
}

pub fn generate_private_key_from_seed(seed: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(seed);
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn generate_private_key() -> String {
    let entropy = generate_entropy();
    let phrase = generate_mnemonic_phrase_from_entropy(entropy.clone());
    let seed = generate_seed_from_mnemonic(&phrase, "");

    let seed_phrase = Seed {
        bytes: seed.clone()
    };

    let seed = format!("{:X}", seed_phrase);

    // println!("PRIVATE_KEY: {:x}", Sha256::digest(seed.as_bytes()));

    let pk = generate_private_key_from_seed(&seed);

    let seed_hex = hex::encode(&entropy);

    println!("\n ENTROPY: {:?}\nPHRASE: {}\nSEED: {}\n SEED HEX: {}\nPRIVATE_KEY: {}", entropy.clone(), phrase, seed, seed_hex, pk);
    pk
}

