use super::*;

#[test]
fn s_box_test() {
    let mut i: u8;
    for i in 0..=255 {
        assert_eq!(S_BOX[i], s_box(&(i as u8)));
    }

    println!("PASS s_box_test");
}
