mod lib;
mod tests;

use lib::{basic_operations::*, key_msg::*, op_modes::*};
use rand::{thread_rng, RngCore};
use std::convert::TryInto;
use std::env;
use std::fs::File;
use std::io::{Read, Result, Write};

fn main() {
    let mut key = [0u8; 16];
    if let Ok(_) = env::var("KEY") {
        let mut rng = thread_rng();
        let mut key = [0u8; 16];
        let mut f = File::create("key").expect("Failed to create key");
        rng.fill_bytes(&mut key);
        f.write(&key).expect("Failed to write key to file");
    } else {
        let mut f = File::open("key").expect("Failed to load key");
        let res = f.read(&mut key).expect("Wrong while key file");
        assert_eq!(res, 16);
    }

    let mut iv = [0u8; 16];
    if let Ok(_) = env::var("IV") {
        let mut rng = thread_rng();
        rng.fill_bytes(&mut iv);
    } else {
        // Just use the one from file
        let mut f = File::open("iv").expect("Failed to read iv");
        f.read(&mut iv).expect("Failed to read file to iv");
    }
    let iv = u128::from_le_bytes(iv);

    if let Ok(_) = env::var("ENC") {
        let toml = "go-common-master.zip";
        let target = "test.cipher";

        //M_row::cbc_build(Key::new(&key), toml, target, iv);
        M_row::ecb_build(Key::new(&key), toml, target);
    //M_row::ctr_build(Key::new(&key), toml, target);
    } else {
        let cipher = "test.cipher";
        let plain = "test.plain";

        //M_row::cbc_destruct(Key::new(&key), cipher, plain, iv);
        M_row::ecb_destruct(Key::new(&key), cipher, plain);
        //M_row::ctr_destruct(Key::new(&key), cipher, plain);
    }
}
