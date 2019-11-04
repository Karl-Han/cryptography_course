use super::lib::{
    attack::{self, expand_to_i, p_q_close, solve_quadratic, ContinuedRatioStream},
    cipher_plain::{Cipher, Plaintext},
    key::{PrimePair, PrivateKey, PublicKey},
    primality_test::{egcd, miller_rabin},
};
use fraction::Ratio;
use num_bigint::{BigInt, BigUint, ParseBigIntError};
use num_traits::{
    identities::{One, Zero},
    sign::Signed,
};
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
    //assert!(pu.authorize("file", &sign.to_bytes_le().1));
}

#[test]
fn attack_p_q_close() {
    let s = "8419248954524000439721779172023134688983838205866625782151550834434276874684863239544369195264071670152656061813873751842115416791829324879655667191724512456544905595733991629887800889255133717212624547817690492648616532902257249552981800714896543008295153051040335475732125114592095784407296265046992475467";
    let n = BigInt::from_str(s).expect("Error when parse from str");
    let (p, q) = p_q_close(&n);
    assert_eq!(p * q, n);
}

#[test]
fn attack_d_too_small() -> Result<(), ParseBigIntError> {
    let n = BigUint::from_str("3351434899016066636045491452890486808714908934340001357148989")?;
    let e = BigUint::from_str("568760665726569109874762046085236437330350817075710631256941")?;

    let pr = attack::pr_d_too_small(e.clone(), n);
    let plaintext = pr.decrypt(Cipher::new(
        "3261683411171403201777854986577156734462063771959389399058422",
    ));
    println!("{}", plaintext);

    let pu = PublicKey::from(pr);
    assert_eq!(BigInt::from(e), pu.e());
    Ok(())
}
