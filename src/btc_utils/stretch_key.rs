extern crate pbkdf2;
extern crate rand;
use hmac::Hmac;
use pbkdf2::pbkdf2;
use sha2::Sha512;

const ITERATIONS: u32 = 2048; // Number of PBKDF2 iterations
const KEY_LENGTH: usize = 64; // Length of the derived key in bytes

// generate [`Seed`][Seed] from [`Mnemonic`][Mnemonic]
pub fn stretch_key(mnemonic: &str, salt: &str) -> [u8; KEY_LENGTH] {
    let mut seed = [0u8; KEY_LENGTH];
    
    let salt = salt.as_bytes();
    // Generate the key using PBKDF2
    pbkdf2::<Hmac<Sha512>>(
        mnemonic.as_bytes(),
        salt,
        ITERATIONS,
        &mut seed,
    );

    seed
}