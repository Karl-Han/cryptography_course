mod lib;
mod tests;

use lib::{basic_operations::*, key_msg::*, op_modes::*};
use rand::{thread_rng, RngCore};
use std::convert::TryInto;
use std::env;
use std::fs::File;
use std::io::{Read, Result, Write};

fn main() {
    let mut rng = thread_rng();
    let mut iv = [0u8; 16];
    rng.fill_bytes(&mut iv);
    let iv: u128 = u128::from_le_bytes(iv);
    if let Ok(_) = env::var("ENC") {
        let toml = "Cargo.toml";
        let target = "test.cipher";
        let key_file = "key";

        let k = Key::new_random();
        let mut f = File::create(key_file).expect("Failed to create `key`");
        f.write(&k.msg()).expect("Error while writing key file");

        M_row::cbc_build(k, toml, target, iv);
    } else {
        let cipher = "test.cipher";
        let plain = "test.plain";
        let key_file = "key";
        let mut f = File::open(key_file).expect("Failed to load key");
        let mut key = [0u8; 16];

        let res = f.read(&mut key).expect("Wrong while read file");
        assert_eq!(res, 16);

        M_row::cbc_destruct(Key::new(&key), cipher, plain, iv);
    }
}
