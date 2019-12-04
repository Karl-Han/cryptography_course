use rand::{thread_rng, RngCore};
use tiny_keccak::{Sha3, Hasher};
use crate::{Keccakf, Hasher as hasher};

#[test]
fn bool_to_i32() {
    assert_eq!(true as i32, 1);
    assert_eq!(false as i32, 0);
}

#[test]
fn l0_test() {
    let expect = [0xa7, 0xff, 0xc6, 0xf8, 0xbf, 0x1e, 0xd7, 0x66, 0x51, 0xc1, 0x47, 0x56, 0xa0, 0x61, 0xd6, 0x62, 0xf5, 0x80, 0xff, 0x4d, 0xe4, 0x3b, 0x49, 0xfa, 0x82, 0xd8, 0x0a, 0x4b, 0x80, 0xf8, 0x43, 0x4a];
    let mut buf = [0u8; 32];
    let mut keccak = Keccakf::new_v256();
    keccak.hash_str("");
    keccak.finalize(&mut buf);

    assert_eq!(buf.len(), expect.len());
    assert_eq!(buf, expect);
}

#[test]
fn l1600_test() {
    let expect = [0x79,0xF3,0x8A,0xDE,0xC5,0xC2,0x03,0x07,0xA9,0x8E,0xF7,0x6E,0x83,0x24,0xAF,0xBF,0xD4,0x6C,0xFD,0x81,0xB2,0x2E,0x39,0x73,0xC6,0x5F,0xA1,0xBD,0x9D,0xE3,0x17,0x87];

    let input = &[0xa3].repeat(200);
    println!("{:02x?}", input);
    let mut buf = [0u8; 32];
    
    let mut keccak = Keccakf::new_v256();
    keccak.hash(input);
    keccak.finalize(&mut buf);

    assert_eq!(buf.len(), expect.len());
    assert_eq!(buf, expect);
}

#[test]
fn random_test(){
    let mut buf = [0u8; 512];
    let mut rng = thread_rng();
    let mut output_tiny = [0u8; 32];
    let mut output = [0u8; 32];

    rng.fill_bytes(&mut buf);
    println!("buf = {:02x?}", &buf[..32]);

    let mut tiny_keccak = Sha3::v256();
    tiny_keccak.update(&buf);
    tiny_keccak.finalize(&mut output_tiny);

    let mut keccak = Keccakf::new_v256();
    keccak.hash(&buf);
    keccak.finalize(&mut output);

    assert_eq!(output_tiny, output);
    println!("output = {:02x?}", output);
}

#[test]
fn specific_str_test(){
    let mut sha3 = Sha3::v256();
    let input_a = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ";
    let mut output_tiny = [0u8; 32];
    let mut output = [0u8; 32];
    sha3.update(input_a);
    sha3.finalize(&mut output_tiny);

    let mut keccak = Keccakf::new_v256();
    keccak.hash(input_a);
    keccak.finalize(&mut output);

    assert_eq!(output, output_tiny);
}
