mod lib;
mod tests;

use lib::basic_operations::*;

fn main() {
    assert_eq!(s_box(&0u8), 0x63);
    assert_eq!(inverse_gf28(&0u8), 0);

    let mut i: u8;
    for i in 0..=255 {
        assert_eq!(S_BOX[i], s_box(&inverse_gf28(&(i as u8))));
    }

    println!("PASS s_box_test");
    println!("Hello, world!");
}
