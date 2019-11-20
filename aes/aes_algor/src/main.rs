mod lib;
mod tests;

use lib::{basic_operations::*, key_msg::*, op_modes::*};
use rand::{thread_rng, RngCore};
use std::convert::TryInto;
use std::env;
use std::fs::File;
use std::io::{Read, Result, Write};

fn main() {
    if let Ok(_) = env::var("KEY") {
        let mut rng = thread_rng();
        let mut key = [0u8; 16];
        let mut f = File::create("key").expect("Failed to create key");
        rng.fill_bytes(&mut key);
        f.write(&key).expect("Failed to write key to file");
    }

    let mut f = File::open("iv").expect("Failed to read iv");
    let mut iv = [0u8; 16];
    f.read(&mut iv).expect("Failed to read file to iv");
    //let mut rng = thread_rng();
    //let mut iv = [0u8; 16];
    //rng.fill_bytes(&mut iv);
    let iv: u128 = u128::from_le_bytes(iv);

    let mut key = [0u8; 16];
    let mut f = File::open("key").expect("Failed to load key");
    let res = f.read(&mut key).expect("Wrong while read file");
    assert_eq!(res, 16);

    if let Ok(_) = env::var("ENC") {
        let toml = "Cargo.toml";
        let target = "test.cipher";

        M_row::cbc_build(Key::new(&key), toml, target, iv);
    } else {
        let cipher = "test.cipher";
        let plain = "test.plain";

        M_row::cbc_destruct(Key::new(&key), cipher, plain, iv);
    }
}
