#![feature(test)]

mod lib;
mod tests;
mod benches;

use lib::{Hasher, Keccakf};

fn rho() {
    let mut back_shift = [0u32; 25];
    let mut x = 1;
    let mut y = 0;
    for t in 0..24 {
        back_shift[5 * y + x] = (((t + 1) * (t + 2)) >>1) %64;
        //println!("t = {}, ({}, {}) = {}",t, x, y, back_shift[5 * y + x]);
        let y_temp = y;
        y = ((x << 1) + y + (y << 1)) %5;
        x = y_temp;
        //println!("new ({}, {})", x, y);
    }

    dbg!(&back_shift);
}

fn main() {
    let mut buf = [0u8; 16];
    let mut keccakf = Keccakf::new_v128();
    let filename = String::from("flac.rar");
    keccakf.hash_file(filename);
    keccakf.finalize(&mut buf);

    println!("{:02x?}", buf);
}
