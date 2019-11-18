mod lib;
mod tests;

use lib::{basic_operations::*, key_msg::*};
use std::convert::TryInto;

fn main() {
    let mut k: [u8; 16] = [
        0x0f, 0x15, 0x71, 0xc9, 0x47, 0xd9, 0xe8, 0x59, 0x0c, 0xb7, 0xad, 0xd6, 0xaf, 0x7f, 0x67,
        0x98,
    ];
    let mut msg: [u8; 16] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32,
        0x10,
    ];

    //k.reverse();
    let k = Key::new(&k);

    let round_keys = k.expansion();

    let mut m: M_matrix = M_row::new(&msg).into();

    println!("Init m = {:02x?}", m);
    m.add_round_key(&Key::from(Vec::from(&round_keys[0..4])));
    println!("Round0, m = {:02x?}", m);

    for i in 0..9 {
        m.sub_s_box();
        println!("Round {}, After sub_s_box m = {:02x?}", i, m);
        m.shitf_rows();
        println!("Round {}, After shift_row m = {:02x?}", i, m);
        m.mix_col();
        println!("Round {}, After mix_col m = {:02x?}", i, m);
        m.add_round_key(&Key::from(Vec::from(&round_keys[4 * (i + 1)..4 * (i + 2)])));
        println!("Round {}, After add round_key m = {:02x?}\n", i, m);
    }

    // the last one
    m.sub_s_box();
    m.shitf_rows();
    m.add_round_key(&Key::from(round_keys[40..44].to_vec()));
    println!("Encrypt result = {:02x?}", m);
}
