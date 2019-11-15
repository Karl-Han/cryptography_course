extern crate rand;

use rand::prelude::*;

#[derive(Debug)]
pub struct Key([u8; 32]);

impl Key {
    pub fn new() -> Key {
        let mut k = [0u8; 32];
        let mut rng = rand::thread_rng();

        rng.fill_bytes(&mut k);
        println!("{:?}", k);
        Key(k)
    }
}
