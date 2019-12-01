mod tests;
mod lib;

use lib::{Keccakf, Hasher};

fn main() {
    let mut buf = [0u8; 32];

    let mut keccakf = Keccakf::new_v256();
    keccakf.hash_str("", &mut buf);

    println!("{:02x?}", buf);
}
