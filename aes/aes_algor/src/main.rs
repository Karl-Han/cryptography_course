mod lib;
mod tests;

use lib::{basic_operations::*, key_msg::*, op_modes::*};
use std::convert::TryInto;
use std::fs::File;
use std::io::Write;

fn main() {
    let toml = "Cargo.toml";
    let target = "test.cipher";
    let key_file = "key";

    let k = Key::new_random();
    let mut f = File::create(key_file).expect("Failed to create `key`");
    f.write(&k.msg());

    M_row::ecb_build(k, toml, target);
}
