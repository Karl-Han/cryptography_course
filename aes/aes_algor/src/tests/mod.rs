use super::*;

#[test]
fn s_box_test() {
    let mut i: u8;
    for i in 0..=255 {
        assert_eq!(S_BOX[i], s_box(&(i as u8)));
    }

    println!("PASS s_box_test");
}

#[test]
fn matrix_multiplication_test() {
    let mut n = M_matrix::new_with_u8(&[[1, 2, 3, 4], [5, 6, 7, 8], [1, 3, 2, 4], [5, 7, 6, 8]]);
    n.mix_col();
    assert_eq!(
        n,
        M_matrix::new_with_u8(&[
            [23, 32, 35, 44],
            [19, 30, 29, 40],
            [23, 35, 32, 44],
            [19, 29, 30, 40]
        ])
    );
}
