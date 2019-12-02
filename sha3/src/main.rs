mod lib;
mod tests;

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
    let mut buf = Vec::new();
    let mut keccakf = Keccakf::new_v256();
    keccakf.hash_str("", &mut buf);

    assert_eq!(buf.len(), 32);
    println!("{:02x?}", buf);
}
