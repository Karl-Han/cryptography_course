extern crate rand;

use crate::lib::basic_operations::*;
use rand::prelude::*;
use std::ops::{Add, Mul, Sub};

pub const MIX_COL: [[u8; 4]; 4] = [[2, 3, 1, 1], [1, 2, 3, 1], [1, 1, 2, 3], [3, 1, 1, 2]];

#[derive(Debug)]
pub struct Key([u8; 32]);

// Key is used to expand
impl Key {
    pub fn new() -> Key {
        let mut k = [0u8; 32];
        let mut rng = rand::thread_rng();

        rng.fill_bytes(&mut k);
        println!("{:?}", k);
        Key(k)
    }
}

//To be improved
//enum M{
//    M_row([u8; 16]),
//    M_matrix([[u8;4]; 4],
//}

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
                println!("col {} = {:?}", k, temp_col_rhs);
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
        println!("matrix before shift:{:?}", self.msg);
        for i in 1..self.msg.len() {
            M_matrix::shift_row(&mut self.msg[i], i);
        }
        println!("matrix after shift:{:?}", self.msg);
    }

    fn shift_row(row: &mut [u8; 4], i: usize) {
        let temp = row.clone();

        for i in 0..4 {
            row[i] = temp[(i + 1) % 4];
        }
    }

    pub fn mix_col(&mut self) {
        let temp = self.clone();
        *self = M_matrix::new_with_u8(&MIX_COL) * temp;
    }

    pub fn msg(&self) -> [[u8; 4]; 4] {
        self.msg.clone()
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
            println!("m[{}] = {}", counter, m.0[counter]);
            counter += 1;
        }
        M_matrix::new_with_u8(&arr)
    }
}
