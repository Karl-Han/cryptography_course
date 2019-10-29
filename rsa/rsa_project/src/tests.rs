extern crate num_bigint;
extern crate num_traits;

use super::lib::{
    cipher_plain::Plaintext,
    key::{PrivateKey, PublicKey},
    primality_test::egcd,
};
use num_bigint::BigInt;
use std::str::FromStr;
use std::string::FromUtf8Error;

#[test]
fn egcd_test() -> Result<(), num_bigint::ParseBigIntError> {
    let mut a = BigInt::from_str("12380339579751423114926012726846665859109128160473940858920447764947665998051717659753451209511089125471702436331586838284243106985194623174305037109060833")?;

    let mut b = BigInt::from_str("12693668861142308203949909107307076109568333642561164188438129182387558715809963097589529080910832736763413534205399930018866988526805104438993153026665709")?;

    let mut r = num_traits::one();
    let mut s = num_traits::zero();
    let gcd = egcd(&mut a, &mut b, &mut r, &mut s).unwrap();
    println!("{} * {} + {} * {} = {}", a, r, b, s, gcd);

    // a * r = 1 (mod b)
    assert_eq!(a.clone() * r.clone() % b.clone(), BigInt::from(1u32));
    Ok(())
}

#[test]
fn encrypt_decrypt_test() -> Result<(), FromUtf8Error> {
    let pr = PrivateKey::new_with_key_size(1024);
    let pu: PublicKey = pr.clone().into();
    //println!("{}", pr);
    //println!("{}", pu);

    let s = "Hello World";
    println!("{:?}", s.as_bytes());
    let cipher = pu.encrypt(Plaintext::new(s.as_bytes()));
    println!("cipher = {}", cipher);

    let m = pr.decrypt(cipher);
    println!(
        "plain = {:?}\n{}",
        m.fragments.to_bytes_le(),
        m.into_string()?
    );
    assert_eq!(s, m.into_string()?);
    Ok(())
}

#[test]
fn sign_authorize_test() {
    let pr = PrivateKey::new_with_key_size(1024);
    let pu = PublicKey::from(pr.clone());

    let sign = pr.sign("file");
    assert!(pu.authorize("file", &sign.to_bytes_le().1));
}
