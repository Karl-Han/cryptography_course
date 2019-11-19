extern crate rand;

use crate::lib::{
    basic_operations::{self, s_box, S_BOX, S_BOX_INV},
    op_modes::ECB_mode,
    Algor::EncDec,
};
use rand::prelude::*;
use std::convert::TryInto;
use std::mem;
use std::ops::{Add, Mul, Sub};

pub const MIX_COL: [[u8; 4]; 4] = [[2, 3, 1, 1], [1, 2, 3, 1], [1, 1, 2, 3], [3, 1, 1, 2]];
pub const MIX_COL_TRANS: [[u8; 4]; 4] = [[2, 1, 1, 3], [3, 2, 1, 1], [1, 3, 2, 1], [1, 1, 3, 2]];

pub const MIX_COL_INV: [[u8; 4]; 4] = [
    [0x0E, 0x0B, 0x0D, 0x09],
    [0x09, 0x0E, 0x0B, 0x0D],
    [0x0D, 0x09, 0x0E, 0x0B],
    [0x0B, 0x0D, 0x09, 0x0E],
];

pub const RC: [u8; 10] = [1, 2, 4, 8, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];
pub const ENCRYPT: usize = 0;
pub const DECRYPT: usize = 2;

#[derive(Debug)]
pub struct Key([u8; 16]);

// Key is used to expand
impl Key {
    pub fn new(arr: &[u8; 16]) -> Key {
        Key(arr.clone())
    }

    pub fn new_random() -> Key {
        let mut rng = rand::thread_rng();

        let mut arr = [0u8; 16];
        rng.fill_bytes(&mut arr);
        println!("Key = {:02x?}", arr);

        Key(arr)
    }

    // Each Word in origin key is regard as BIG endian
    // Words is regard as LITTLE endian
    pub fn expansion(&self) -> Vec<u32> {
        // implement the default case with key is 128bits
        let mut v: Vec<u32> = Vec::new();

        // init first four element in v
        for i in 0..4 {
            let mut arr = [0u8; 4];
            for j in 0..4 {
                arr[j] = self.0[4 * i + j];
            }
            arr.reverse();
            let word: u32 = u32::from_le_bytes(arr);
            //println!("w{:<3} = {:08x}", i, word);
            v.push(word);
        }

        // the rest 40 words
        for i in 4..44 {
            let mut temp = v[i - 1].clone();

            if i % 4 == 0 {
                // temp is s_box(shift_row(temp, 1))
                let mut t = temp.to_be_bytes();
                M_matrix::shift_row(&mut t, 1);
                //println!("Rot(w{0}) = x{0:<} = {1:x?}", i - 1, t);
                let mut t: Vec<u8> = t.iter().map(|x| s_box(&x)).collect();
                //println!("S_BOX(x{0:<}) = y{0:<} = {1:x?}", i - 1, t);
                //println!("RC{:02} = {:02x} 00 00 00", (i - 1) / 4, RC[(i - 1) / 4]);
                t[0] ^= RC[(i - 1) / 4];
                //println!("y{0:<} ^ RC{1:02} = {2:x?}", i - 1, RC[(i - 1) / 4], t);
                let mut bytes = [0u8; 4];
                for (i, item) in t.iter().enumerate() {
                    bytes[i] = item.clone();
                }
                //bytes.reverse();
                temp = u32::from_be_bytes(bytes);
            }

            let res = v[i - 4] ^ temp;
            //println!("w{:<3} = {:08x}", i, res);
            v.push(res);
        }

        v
    }

    pub fn from(arr: Vec<u32>) -> Self {
        // to be discuss the order of the key and round_keys
        assert_eq!(arr.len(), 4);
        //println!("arr = {:08x?}", arr);

        let t: Vec<[u8; 4]> = arr.iter().map(|x| x.to_be_bytes()).collect();
        //println!("t = {:?}", t);
        let t: [[u8; 4]; 4] = t.as_slice()[..]
            .try_into()
            .expect("Failed to convert because of length");

        let r: M_row = M_matrix::new_with_u8(&t).into();
        Self(r.0)
    }

    pub fn msg(&self) -> [u8; 16] {
        self.0.clone()
    }
}

//To be improved
//enum M{
//    M_row([u8; 16]),
//    M_matrix([[u8;4]; 4],
//}

// Start to think about the necessity of M_row
#[derive(Debug)]
pub struct M_row([u8; 16]);

impl EncDec for M_row {
    fn new(s: [u8; 16]) -> M_row {
        M_row::new(s)
    }

    fn encrypt(self, key: &Key) -> [u8; 16] {
        let mut m: M_matrix = self.into();
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
        res.0
    }

    fn decrypt(self, key: &Key) -> [u8; 16] {
        // m is now cipher text
        let mut m: M_matrix = self.into();
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
        res.0
    }

    fn msg(&self) -> [u8; 16] {
        self.0.clone()
    }
}

impl M_row {
    pub fn new(msg: [u8; 16]) -> M_row {
        M_row(msg)
    }
    // it is a very wasteful way to do this
    // To be deprecated
    pub fn sub_s_box_cal(&mut self) {
        for (i, item) in self.0.clone().iter().enumerate() {
            self.0[i] = s_box(item);
        }
    }

    pub fn sub_s_box(&mut self) {
        for (i, item) in self.0.clone().iter().enumerate() {
            self.0[i] = S_BOX[(*item as usize)];
        }
    }

    pub fn msg(&self) -> [u8; 16] {
        self.0
    }
}

impl ECB_mode for M_row {
    type T = M_row;
}

trait Matrix {
    fn add(&self, m: &Self) -> Self;
    fn sub(&self, m: &Self) -> Self;
    fn multiply(&self, m: &Self) -> Self;
    fn inverse(&self) -> Self;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M_matrix {
    msg: [[u8; 4]; 4],
}

impl Add for M_matrix {
    type Output = M_matrix;

    fn add(self, rhs: M_matrix) -> M_matrix {
        let len_row = self.msg.len();
        let len_col = self.msg[0].len();
        let mut temp = M_matrix::new();

        assert_eq!(len_row, rhs.msg.len());
        assert_eq!(len_col, rhs.msg[0].len());

        for i in 0..len_row {
            for j in 0..len_col {
                temp.msg[i][j] = self.msg[i][j] + rhs.msg[i][j];
            }
        }

        temp
    }
}

impl Sub for M_matrix {
    type Output = M_matrix;

    fn sub(self, rhs: M_matrix) -> M_matrix {
        let len_row = self.msg.len();
        let len_col = self.msg[0].len();
        let mut temp = M_matrix::new();

        assert_eq!(len_row, rhs.msg.len());
        assert_eq!(len_col, rhs.msg[0].len());

        for i in 0..len_row {
            for j in 0..len_col {
                temp.msg[i][j] = self.msg[i][j] - rhs.msg[i][j];
            }
        }

        temp
    }
}

impl Mul for M_matrix {
    type Output = M_matrix;

    fn mul(self, rhs: M_matrix) -> M_matrix {
        let len_row = self.msg.len();
        let len_col = self.msg[0].len();
        // in this case, nothing new size of matrix is introduced
        let mut temp = M_matrix::new();

        // match this is enough
        assert_eq!(len_row, rhs.msg[0].len());

        //for m in 0..len_col {
        //    //println!("{:?}", rhs.msg[m][1]);
        //}

        for k in 0..len_col {
            for i in 0..len_row {
                let mut temp_col_rhs = Vec::new();

                // get col_k of rhs
                for t in 0..len_col {
                    // copy the col k in rhs to temp_col_rhs
                    temp_col_rhs.push(rhs.msg[t][k]);
                }
                //println!("col {} = {:02x?}", k, temp_col_rhs);
                let temp_col_rhs = temp_col_rhs.as_slice();

                //println!("row{} = {:02x?}", i, self.msg[i]);
                let mut res: u16 = 0;
                for t in 0..len_col {
                    res ^= basic_operations::multiply(
                        &(self.msg[i][t] as u16),
                        &(temp_col_rhs[t] as u16),
                    );
                }
                temp.msg[i][k] = res.to_le_bytes()[0];
                //println!("temp = {:?}", temp.msg[i][k]);
            }
        }

        temp
    }
}

impl Matrix for M_matrix {
    fn add(&self, m: &Self) -> Self {
        self.clone() + m.clone()
    }
    fn sub(&self, m: &Self) -> Self {
        self.clone() - m.clone()
    }
    fn multiply(&self, m: &Self) -> Self {
        self.clone() * m.clone()
    }
    fn inverse(&self) -> Self {
        panic!("DID NOT IMPLEMENTED IN M_matrix");
    }
}

impl M_matrix {
    pub fn new() -> Self {
        Self { msg: [[0u8; 4]; 4] }
    }

    pub fn new_with_u8(msg: &[[u8; 4]; 4]) -> Self {
        Self { msg: msg.clone() }
    }

    pub fn shift_rows(&mut self, mode: usize) {
        //println!("matrix before shift:{:?}", self.msg);
        if mode == ENCRYPT {
            for i in 1..self.msg.len() {
                M_matrix::shift_row(&mut self.msg[i], i);
            }
        } else {
            for i in 1..self.msg.len() {
                M_matrix::shift_row(&mut self.msg[i], 4 - i);
            }
        }
        //println!("matrix after shift:{:?}", self.msg);
    }

    pub fn shift_row(row: &mut [u8; 4], i: usize) {
        let temp = row.clone();

        //println!("temp = {:x?}", temp);
        for j in 0..4 {
            row[j] = temp[(i + j) % 4];
        }
        //println!("After shift = {:x?}", row);
    }

    pub fn mix_col(&mut self, mode: usize) {
        let temp = self.clone();
        if mode == ENCRYPT {
            *self = M_matrix::new_with_u8(&MIX_COL) * temp;
        } else {
            *self = M_matrix::new_with_u8(&MIX_COL_INV) * temp;
        }
    }

    pub fn msg(&self) -> [[u8; 4]; 4] {
        self.msg.clone()
    }

    pub fn add_round_key(&mut self, round_key: &Key) {
        // Key is [u8; 16]
        //println!("M_matrix = {:x?}", self.msg);
        //println!("Key = {:02x?}", round_key.0);

        // it needs to be the transpose of key
        // because its order is different
        // Or I need to change the structure of expansion
        for (i, ele) in round_key.0.iter().enumerate() {
            // Before
            //self.msg[i / 4][i % 4] ^= ele;

            // Now
            self.msg[i % 4][i / 4] ^= ele;
        }

        //println!("After round_key = {:x?}", self.msg);
    }

    pub fn sub_s_box(&mut self, mode: usize) {
        if mode == ENCRYPT {
            for (i, item) in self.msg.clone().iter().enumerate() {
                // for rows in self.msg
                for (j, item) in item.iter().enumerate() {
                    mem::replace(&mut self.msg[i][j], S_BOX[*item as usize]);
                }
            }
        } else {
            for (i, item) in self.msg.clone().iter().enumerate() {
                // for rows in self.msg
                for (j, item) in item.iter().enumerate() {
                    mem::replace(&mut self.msg[i][j], S_BOX_INV[*item as usize]);
                }
            }
        }
    }

    pub fn transpose(&mut self) {
        let mut temp = M_matrix::new();
        //println!("self before transpose = {:02x?}", self.msg());

        for (i, item) in self.msg.iter().enumerate() {
            // get every row in self
            for (j, item) in item.iter().enumerate() {
                temp.msg[j][i] = *item;
            }
        }

        *self = temp;
        //println!("self after transpose = {:02x?}", self.msg());
    }
}

impl From<M_matrix> for M_row {
    fn from(m: M_matrix) -> Self {
        let mut arr = [0u8; 16];
        let mut counter = 0;
        for row in &m.msg {
            for ele in row {
                arr[counter] = ele.clone();
                counter += 1;
            }
        }
        M_row::new(arr)
    }
}

impl From<M_row> for M_matrix {
    fn from(m: M_row) -> Self {
        let mut arr = [[0u8; 4]; 4];
        //let mut counter = 0;
        //while counter < 16 {
        //    arr[counter / 4][counter % 4] = m.0[counter];
        //    //println!("m[{}] = {}", counter, m.0[counter]);
        //    counter += 1;
        //}
        // i + 4*j is the index
        for i in 0..4 {
            for j in 0..4 {
                arr[i][j] = m.0[i + 4 * j];
            }
        }
        M_matrix::new_with_u8(&arr)
    }
}
