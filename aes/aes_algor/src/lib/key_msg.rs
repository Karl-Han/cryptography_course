extern crate rand;

use crate::lib::basic_operations::*;
use rand::prelude::*;
use std::convert::TryInto;
use std::mem;
use std::ops::{Add, Mul, Sub};

pub const MIX_COL: [[u8; 4]; 4] = [[2, 3, 1, 1], [1, 2, 3, 1], [1, 1, 2, 3], [3, 1, 1, 2]];

pub const RC: [u8; 10] = [1, 2, 4, 8, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];

#[derive(Debug)]
pub struct Key([u8; 16]);

//impl From<[u32; 4]> for Key {
//    fn from(u: [u32; 4]) -> Self {
//        // to be discuss the order of key and round_keys
//        unimplemented!();
//    }
//}

// Key is used to expand
impl Key {
    pub fn new(arr: &[u8; 16]) -> Key {
        Key(arr.clone())
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
}

//To be improved
//enum M{
//    M_row([u8; 16]),
//    M_matrix([[u8;4]; 4],
//}

// Start to think about the necessity of M_row
pub struct M_row([u8; 16]);

impl M_row {
    pub fn new(msg: &[u8; 16]) -> M_row {
        M_row(msg.clone())
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

        for m in 0..len_col {
            println!("{:?}", rhs.msg[m][1]);
        }

        for k in 0..len_col {
            for i in 0..len_row {
                let mut temp_col_rhs = Vec::new();

                // get col_k of rhs
                for t in 0..len_col {
                    // copy the col k in rhs to temp_col_rhs
                    temp_col_rhs.push(rhs.msg[t][k]);
                }
                //println!("col {} = {:?}", k, temp_col_rhs);
                let temp_col_rhs = temp_col_rhs.as_slice();

                for t in 0..len_col {
                    temp.msg[i][k] += self.msg[i][t] * temp_col_rhs[t];
                }
                //println!("temp = {:?}", temp);
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

    pub fn shitf_rows(&mut self) {
        //println!("matrix before shift:{:?}", self.msg);
        for i in 1..self.msg.len() {
            M_matrix::shift_row(&mut self.msg[i], i);
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

    pub fn mix_col(&mut self) {
        let temp = self.clone();
        *self = M_matrix::new_with_u8(&MIX_COL) * temp;
    }

    pub fn msg(&self) -> [[u8; 4]; 4] {
        self.msg.clone()
    }

    pub fn add_round_key(&mut self, round_key: &Key) {
        // Key is [u8; 16]
        //println!("M_matrix = {:x?}", self.msg);
        //println!("Key = {:x?}", round_key.0);

        for (i, ele) in round_key.0.iter().enumerate() {
            self.msg[i / 4][i % 4] ^= ele;
        }

        //println!("After round_key = {:x?}", self.msg);
    }

    pub fn sub_s_box(&mut self) {
        for (i, item) in self.msg.clone().iter().enumerate() {
            // for rows in self.msg
            for (j, item) in item.iter().enumerate() {
                mem::replace(&mut self.msg[i][j], S_BOX[*item as usize]);
            }
        }
        //*self = M_matrix::new();
        //unimplemented!();
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
        M_row::new(&arr)
    }
}

impl From<M_row> for M_matrix {
    fn from(m: M_row) -> Self {
        let mut arr = [[0u8; 4]; 4];
        let mut counter = 0;
        while counter < 16 {
            arr[counter / 4][counter % 4] = m.0[counter];
            //println!("m[{}] = {}", counter, m.0[counter]);
            counter += 1;
        }
        M_matrix::new_with_u8(&arr)
    }
}
