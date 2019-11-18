use crate::lib::key_msg::*;

pub const ENCRYPT: usize = 0;
pub const DECRYPT: usize = 2;

// use M_row as input, just like [u8; 16]
pub fn encrypt(m: M_row, key: Key) -> M_row {
    let mut m: M_matrix = m.into();
    let round_keys = key.expansion();

    //println!("Init m = {:02x?}", m);
    m.add_round_key(&Key::from(Vec::from(&round_keys[0..4])));
    //println!("Round0, m = {:02x?}", m);

    for i in 0..9 {
        m.sub_s_box(ENCRYPT);
        //println!("Round {}, After sub_s_box m = {:02x?}", i, m);
        m.shift_rows(ENCRYPT);
        //println!("Round {}, After shift_row m = {:02x?}", i, m);
        m.mix_col(ENCRYPT);
        //println!("Round {}, After mix_col m = {:02x?}", i, m);
        m.add_round_key(&Key::from(Vec::from(&round_keys[4 * (i + 1)..4 * (i + 2)])));
        //println!("Round {}, After add round_key m = {:02x?}\n", i, m);
    }

    // the last round
    m.sub_s_box(ENCRYPT);
    m.shift_rows(ENCRYPT);
    m.add_round_key(&Key::from(round_keys[40..44].to_vec()));
    //println!("Encrypt result = {:02x?}", m);
    m.transpose();

    let res: M_row = m.into();
    res
}

pub fn decrypt(m: M_row, key: Key) -> M_row {
    // m is now cipher text
    let mut m: M_matrix = m.into();
    let round_keys = key.expansion();

    //println!("Init m = {:02x?}", m);
    m.add_round_key(&Key::from(Vec::from(&round_keys[40..44])));
    //println!("Round0, m = {:02x?}", m);

    for reverse in 0..9 {
        let i = 8 - reverse;
        m.shift_rows(DECRYPT);
        //println!("Round {}, After inverse shift_row m = {:02x?}", i, m);
        m.sub_s_box(DECRYPT);
        //println!("Round {}, After inverse sub_s_box m = {:02x?}", i, m);
        m.add_round_key(&Key::from(Vec::from(&round_keys[4 * (i + 1)..4 * (i + 2)])));
        //println!("Round {}, After add inverse round_key m = {:02x?}\n", i, m);
        m.mix_col(DECRYPT);
        //println!("Round {}, After inverse mix_col m = {:02x?}", i, m);
    }

    // the last round
    m.sub_s_box(DECRYPT);
    m.shift_rows(DECRYPT);
    m.add_round_key(&Key::from(round_keys[0..4].to_vec()));
    //println!("Decrypt result = {:02x?}", m);
    m.transpose();

    let res: M_row = m.into();
    res
}
