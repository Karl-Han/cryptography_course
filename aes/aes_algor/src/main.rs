mod lib;
mod tests;

use lib::{basic_operations::*, key_msg::*};

fn main() {
    //assert_eq!(s_box(&0u8), 0x63);
    //assert_eq!(inverse_gf28(&0u8), 0);

    //let mut i: u8;
    //for i in 0..=255 {
    //    assert_eq!(S_BOX[i], s_box(&inverse_gf28(&(i as u8))));
    //}

    //println!("PASS s_box_test");
    //let matrix: [[u8; 4]; 4] = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 0, 10, 11], [12, 13, 14, 15]];
    //let m: M_row = M_matrix::new_with_u8(&matrix).into();
    //let mut m: M_matrix = m.into();

    //m.shitf_rows();

    //let new_matrix = m.clone() + m.clone();
    //println!("{:?}", new_matrix);

    //let a = m.clone() * m.clone();

    //println!("{:?}", a);

    //println!("Before mix col : {:?}", m.clone().msg());
    //m.mix_col();

    let n = M_matrix::new_with_u8(&[[1, 2, 3, 4], [5, 6, 7, 8], [1, 3, 2, 4], [5, 7, 6, 8]]);
    assert_eq!(
        M_matrix::new_with_u8(&MIX_COL) * n,
        M_matrix::new_with_u8(&[
            [23, 32, 35, 44],
            [19, 30, 29, 40],
            [23, 35, 32, 44],
            [19, 29, 30, 40]
        ])
    );
    //println!("After mix col : {:?}", m.msg());
}
