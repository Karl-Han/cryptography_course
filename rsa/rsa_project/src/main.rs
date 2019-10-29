extern crate clap;
extern crate num_bigint as bigint;
extern crate num_traits;
extern crate rand;
extern crate sha3;

use bigint::*;
use clap::{App, Arg, SubCommand};
use num_traits::cast::ToPrimitive;
use primality_test::*;
use rand::distributions::uniform::UniformSampler;
use rand::RngCore;
use sha3::{Digest, Sha3_256};
use std::fmt;
use std::fs::File;
use std::io::{self, prelude::*};
use std::str::FromStr;
use std::string::{FromUtf8Error, String};

pub mod primality_test {
    use bigint::*;
    use core::ops::BitAnd;
    use rand::distributions::uniform::UniformSampler;
    fn pass_miller_rabin(num: &BigInt, modulus: &BigInt) -> bool {
        let phi = modulus.clone() - 1u32;
        let mut odd: BigInt = phi.clone();

        while odd.clone().bitand(&num_traits::one()) == num_traits::one() {
            odd = odd / 2u32;
        }

        let mut base = num.modpow(&odd, &modulus);

        while phi >= odd {
            if base == num_traits::One::one() || base == num.clone() - 1u32 {
                return true;
            }
            base = base.modpow(&BigInt::from(2u32), &modulus);
            odd = odd * 2u32;
        }

        return false;
    }

    pub fn miller_rabin(num: &BigInt, times: usize) -> bool {
        if num.bitand(BigInt::from(1u32)) != BigInt::from(1u32) {
            return false;
        }
        let mut rng = rand::thread_rng();

        let big_int_gen = UniformBigInt::new(num_traits::One::one(), num - 1u32);

        for _ in 0..times {
            if !pass_miller_rabin(&big_int_gen.sample(&mut rng), &num) {
                return false;
            }
        }
        return true;
    }
}

pub fn generate_with_head(head: u64, size: usize) -> BigInt {
    let low = BigInt::from(head) << (size - 32);
    let high = BigInt::from(1u32) << size;
    //println!("low = {} with bits {}", low, low.bits());
    //println!("high = {} with bits {}", high, high.bits());

    let sampler: UniformBigInt;
    if low < high {
        sampler = UniformSampler::new(low, high);
    } else {
        sampler = UniformSampler::new(high, low);
    }
    let mut rng = rand::thread_rng();
    let mut sample: BigInt;

    loop {
        //sample = gen.sample(&mut rng);
        sample = sampler.sample(&mut rng);
        //println!("{} {}", sample, sample.bits());

        if sample.bits() == size && miller_rabin(&sample, 50) {
            //println!("{} {}", sample, sample.bits());
            break;
        }
    }

    return sample;
}

impl fmt::Display for PrimePair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "p : {}\nq : {}", self.p, self.q)
    }
}
#[derive(Clone)]
struct PrimePair {
    p: BigInt,
    q: BigInt,
}

impl PrimePair {
    pub fn new(psize: usize, qsize: usize) -> PrimePair {
        let mut rng = rand::thread_rng();

        let mut phead: u64 = rng.next_u32() as u64;
        let mut qhead: u64 = rng.next_u32() as u64;
        loop {
            phead = phead | (1 << 31);
            qhead = qhead | (1 << 31);
            let res: u64 = phead * qhead;

            if res >= 1 << 63 {
                //println!("phead = {}", phead);
                //println!("qhead = {}", qhead);
                //println!("res= {}", res);
                break;
            }
        }

        let psample = generate_with_head(phead, psize);
        let qsample = generate_with_head(qhead, qsize);

        PrimePair {
            p: psample,
            q: qsample,
        }
    }
    pub fn product(&self) -> BigInt {
        return self.p.clone() * self.q.clone();
    }
    pub fn phi_p(&self) -> BigInt {
        return (self.p.clone() - 1u32) * (self.q.clone() - 1u32);
    }
    pub fn p(&self) -> BigInt {
        return self.p.clone();
    }
    pub fn q(&self) -> BigInt {
        return self.q.clone();
    }
    pub fn from_p_q(p: BigInt, q: BigInt) -> Self {
        PrimePair { p, q }
    }
}

impl fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PrivateKey: [{}\nd : {}\ne : {}\nn : {}]",
            self.p_q_pair, self.d, self.e, self.n
        )
    }
}

#[derive(Clone)]
struct PrivateKey {
    p_q_pair: PrimePair,
    //phi_n: BigInt,
    e: BigInt,
    d: BigInt,
    n: BigInt,
}

impl PrivateKey {
    pub fn new(pair: PrimePair, e: BigInt) -> Self {
        // e is the one chosen to be part of PU
        // phi p is bigger than e
        // so it becomes phi_p * r + e * s = 1
        let mut a = e.clone();
        let mut b = pair.phi_p();
        let mut r: BigInt = num_traits::one();
        let mut s: BigInt = num_traits::zero();

        if a < b {
            swap(&mut a, &mut b);
        }
        let res = egcd(&mut a, &mut b, &mut r, &mut s).unwrap();

        //println!("a = {}\nr = {}\nb = {}", a, r, b);
        assert_eq!(res, BigInt::from(1u32));

        s = s + a.clone();
        s = s % a.clone();
        //println!(
        //    "phi p = {}\nres = {}",
        //    pair.phi_p(),
        //    (b.clone() * s.clone()) % pair.phi_p()
        //);

        assert_eq!((b.clone() * s.clone()) % pair.phi_p(), BigInt::from(1u32));

        // r is e now
        let n = pair.product();
        return PrivateKey {
            p_q_pair: pair,
            //phi_n : pair.product()
            e,
            d: s,
            n,
        };
    }
    pub fn default_new(pair: PrimePair) -> Self {
        return Self::new(pair, BigInt::from(65537u32));
    }
    pub fn new_with_dn(d: BigInt, n: BigInt) -> Self {
        PrivateKey {
            p_q_pair: PrimePair::from_p_q(num_traits::zero(), num_traits::zero()),
            e: num_traits::zero(),
            d,
            n,
        }
    }
    pub fn new_with_key_size(size: usize) -> Self {
        let p = size / 2;
        let q = size - p;
        return Self::default_new(PrimePair::new(p, q));
    }
    pub fn e(&self) -> BigInt {
        return self.e.clone();
    }
    pub fn product(&self) -> BigInt {
        return self.p_q_pair.product();
    }
    pub fn decrypt(&self, c: Cipher) -> Plaintext {
        let res = if c.fragments.bits() <= 1024 {
            c.fragments.modpow(&self.d, &self.n)
        } else {
            println!("{} with {} bits", c, c.fragments.bits());
            panic!("Cipher too long");
        };

        return Plaintext { fragments: res };
    }
    pub fn from_biguint(p: BigInt, q: BigInt) -> Self {
        return Self::default_new(PrimePair::from_p_q(p, q));
    }
    pub fn sign(&self, filename: &str) -> BigInt {
        let mut file = File::open(filename).expect("Unable to open file");
        let mut hasher = Sha3_256::new();

        io::copy(&mut file, &mut hasher);

        let result = hasher.result();
        return BigInt::from_bytes_le(Sign::Plus, &result);
    }
}

pub fn swap(a: &mut BigInt, b: &mut BigInt) {
    let temp = a.clone();
    *a = b.clone();
    *b = temp;
}

pub fn egcd(a: &mut BigInt, b: &mut BigInt, r: &mut BigInt, s: &mut BigInt) -> Option<BigInt> {
    let flag: bool = a.clone() < b.clone();
    if flag {
        swap(a, b);
    }

    let mut temp_a: BigInt = a.clone();
    let mut temp_b: BigInt = b.clone();

    let mut r1: BigInt = num_traits::one();
    let mut r2: BigInt = num_traits::zero();
    let mut s1: BigInt = num_traits::zero();
    let mut s2: BigInt = num_traits::one();

    while temp_b.clone() != num_traits::zero() {
        // when temp_b == zero, a gets what we need
        let q = temp_a.clone() / temp_b.clone();
        temp_a = temp_a.clone() % temp_b.clone();

        r1 = r1 - q.clone() * s1.clone();
        r2 = r2 - q.clone() * s2.clone();

        swap(&mut r1, &mut s1);
        swap(&mut r2, &mut s2);
        swap(&mut temp_a, &mut temp_b);
    }

    *r = r1.clone();
    *s = r2.clone();

    //println!("{} * {} + {} * {} = {}", a, r1, b, r2, temp_a);
    return Some(temp_a);
}

struct Cipher {
    // fixed length per element
    pub fragments: BigInt,
}

impl Cipher {
    pub fn new(s: &str) -> Cipher {
        let num = BigInt::from_str(s).expect("Unable to parse Cipher from str");
        //println!("Cipher {} with {} bits", num, num.bits());

        Cipher { fragments: num }
    }
}

#[derive(Debug)]
struct Plaintext {
    pub fragments: BigInt,
}

impl Plaintext {
    pub fn new(s: &str) -> Self {
        Plaintext {
            fragments: BigInt::from_bytes_le(Sign::Plus, s.as_bytes()),
        }
    }
    pub fn into_string(&self) -> Result<String, FromUtf8Error> {
        let (_, bytes) = self.fragments.to_bytes_le();
        return String::from_utf8(bytes.to_vec());
    }
}

impl fmt::Display for Plaintext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fragments)
    }
}

impl fmt::Display for Cipher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fragments)
    }
}

#[derive(Debug)]
struct PublicKey {
    e: BigInt,
    n: BigInt,
}

impl From<Vec<u8>> for PublicKey {
    fn from(arr: Vec<u8>) -> PublicKey {
        let key_length_arr = &arr[0..4];
        let key_length = BigUint::from_bytes_le(key_length_arr).to_usize().unwrap();
        let n_arr = &arr[4..key_length];
        let e_arr = &arr[key_length..];

        let n = BigUint::from_bytes_le(n_arr).to_bigint().unwrap();
        let e = BigUint::from_bytes_le(e_arr).to_bigint().unwrap();
        PublicKey { e, n }
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PublicKey:[e = {}\nn = {}]", self.e, self.n)
    }
}

impl PublicKey {
    pub fn encrypt(&self, m: Plaintext) -> Cipher {
        let res = if m.fragments.bits() < 1024 {
            m.fragments.modpow(&self.e, &self.n)
        } else {
            println!("{} with {} bits", m, m.fragments.bits());
            panic!("Plaintext too long");
        };

        return Cipher { fragments: res };
    }
    pub fn from_u8_slice(arr: &mut [u8], key_size: &usize) -> Self {
        let n_arr = &arr[0..*key_size];
        let e_arr = &arr[*key_size..];

        let e = BigInt::from_bytes_le(Sign::Plus, e_arr);
        let n = BigInt::from_bytes_le(Sign::Plus, n_arr);

        PublicKey { e, n }
    }

    pub fn into_vec(&self) -> Vec<u8> {
        let e_arr = self.e.to_biguint().unwrap().to_bytes_le();
        let n_arr = self.n.to_biguint().unwrap().to_bytes_le();

        let mut vec = n_arr.to_vec();
        vec.extend(e_arr.to_vec().iter());
        return vec;
    }
    pub fn from_e_n(e: BigInt, n: BigInt) -> Self {
        PublicKey { e, n }
    }
    pub fn authorize(&self, filename: &str, text: &str) -> bool {
        let mut file = File::open(filename).expect("Unable to open file");
        let mut hasher = Sha3_256::new();

        io::copy(&mut file, &mut hasher);

        let result = hasher.result();
        let res = BigInt::from_bytes_le(Sign::Plus, &result);
        let text = BigInt::from_str(text).expect("Unable to parse text into BigInt");

        return res == text;
    }
}

impl From<PrivateKey> for PublicKey {
    fn from(pr: PrivateKey) -> Self {
        return PublicKey {
            e: pr.e(),
            n: pr.product(),
        };
    }
}

fn argument_parse() -> io::Result<()> {
    let matches = App::new("RSA in Rust")
        .subcommand(
            SubCommand::with_name("encrypt")
                .arg(Arg::with_name("auth").short("a").long("auth"))
                .arg(Arg::with_name("new").long("new").help("Use new key pair"))
                .arg(
                    Arg::with_name("e")
                        .short("e")
                        .long("e-part")
                        .takes_value(true)
                        .help("Exp part of public key"),
                )
                .arg(
                    Arg::with_name("n")
                        .short("n")
                        .long("n-part")
                        .takes_value(true)
                        .help("Modulus part of the public key"),
                )
                .arg(
                    Arg::with_name("text")
                        .short("t")
                        .long("text")
                        .takes_value(true)
                        .help("Text to be encrypt"),
                )
                .arg(
                    Arg::with_name("filename")
                        .short("f")
                        .long("filename")
                        .takes_value(true)
                        .help("File contains plaintext to encrypt"),
                )
                .arg(
                    Arg::with_name("output_filename")
                        .short("o")
                        .long("output")
                        .takes_value(true)
                        .help("Specify the output filename."),
                )
                .arg(
                    Arg::with_name("key_size")
                        .short("l")
                        .long("key-length")
                        .takes_value(true)
                        .help("Specify the length of key"),
                )
                .arg(
                    Arg::with_name("key_file")
                        .short("k")
                        .long("key-file")
                        .takes_value(true)
                        .help("Specify the file contains public key"),
                ),
        )
        .subcommand(
            SubCommand::with_name("decrypt")
                .arg(Arg::with_name("sign").short("s").long("sign"))
                .arg(
                    Arg::with_name("d")
                        .short("d")
                        .long("d-part")
                        .takes_value(true)
                        .help("Exp part of private key"),
                )
                .arg(
                    Arg::with_name("n")
                        .short("n")
                        .long("n-part")
                        .takes_value(true)
                        .help("Modulus part of the private key"),
                )
                .arg(
                    Arg::with_name("cipher")
                        .short("ci")
                        .long("cipher")
                        .takes_value(true)
                        .help("Cipher to be decrypt"),
                )
                .arg(
                    Arg::with_name("filename")
                        .short("f")
                        .long("filename")
                        .takes_value(true)
                        .help("File contains cipher to decrypt"),
                )
                .arg(
                    Arg::with_name("output_filename")
                        .short("o")
                        .long("output")
                        .takes_value(true)
                        .help("Specify the output filename."),
                )
                .arg(
                    Arg::with_name("key_file")
                        .short("k")
                        .long("key-file")
                        .takes_value(true)
                        .help("Specify the file contains public key"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("encrypt", Some(sub_matches)) => {
            /*
            let filename = sub_matches.value_of("filename").unwrap();
            let output_filename: Option<&str> = sub_matches.value_of("output_filename");
            let key_size = sub_matches.value_of("key_size").unwrap();

            let mut file = File::open(filename)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;

            if sub_matches.is_present("new") {
                println!("Generating new key pair...");
                let private_key = PrivateKey::new_with_key_size(
                    FromStr::from_str(key_size).expect(&format!("Invalid key size : {}", key_size)),
                );

                println!("{}", private_key);
                let pu: PublicKey = private_key.into();
                let cipher = pu.encrypt(Plaintext::new(&content));
                println!("Cipher = {}", cipher);
            } else {
                let key_file = sub_matches
                    .value_of("key_file")
                    .expect("No key size and public key file.");
                let mut pu_content = [0u8; 2048];
                let key_length = File::open(key_file)?
                    .read(&mut pu_content)
                    .expect("Error while reading key file in ENCRYPT");
                let pu = PublicKey::from_u8_slice(
                    &mut pu_content,
                    &FromStr::from_str(key_size).expect("Unable to parse key size in ENCRYPT"),
                );
                let cipher = pu.encrypt(Plaintext::new(&content));
                println!("Cipher = {}", cipher);
            }
            */
            if sub_matches.is_present("new") {
                let key_size = sub_matches.value_of("key_size").unwrap();
                let pr = PrivateKey::new_with_key_size(
                    FromStr::from_str(key_size).expect("Unable to parse key_size in ENCRYPT"),
                );
                println!("{}", pr);
                let pu = PublicKey::from(pr);

                println!("{}", pu);
                return Ok(());
            }
            let e = sub_matches.value_of("e").unwrap();
            let n = sub_matches.value_of("n").unwrap();
            let text = sub_matches.value_of("text").unwrap();

            let pu = PublicKey::from_e_n(
                BigInt::from_str(e).expect("Unable to parse e-part"),
                BigInt::from_str(n).expect("Unable to parse n-part"),
            );
            if sub_matches.is_present("auth") {
                // authorize process with just public key
                let filename = sub_matches.value_of("filename").unwrap();
                if pu.authorize(filename, text) {
                    println!("Match signature of {}", filename);
                } else {
                    println!("WRONG signature of {}", filename);
                }
            }

            let cipher = pu.encrypt(Plaintext::new(text));
            println!("{}", cipher);
            Ok(())
        }
        ("decrypt", Some(sub_matches)) => {
            let d = sub_matches.value_of("d").unwrap();
            let n = sub_matches.value_of("n").unwrap();
            if sub_matches.is_present("sign") {
                let filename = sub_matches.value_of("filename").unwrap();
                let pr = PrivateKey::new_with_dn(
                    BigInt::from_str(d).expect("Unable to parse d-part in DECRYPT"),
                    BigInt::from_str(n).expect("Unable to parse n-part in DECRYPT"),
                );

                let signature = pr.sign(filename);
                println!("{}", signature);
                return Ok(());
            }
            let cipher = sub_matches.value_of("cipher").unwrap();
            let pr = PrivateKey::new_with_dn(
                BigInt::from_str(d).expect("Unable to parse d-part"),
                BigInt::from_str(n).expect("Unable to parse n-part"),
            );

            let plaintext = pr.decrypt(Cipher::new(cipher));
            println!("{}", plaintext);

            let s = plaintext
                .into_string()
                .expect("Unable to parse plaintext to string");
            println!("{}", s);
            Ok(())
        }
        _ => Ok(()),
    }
}

fn main() {
    argument_parse();
}

#[cfg(test)]
mod test {
    use super::*;
    use bigint::*;
    #[test]
    fn egcd_test() -> Result<(), bigint::ParseBigIntError> {
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
        println!("{}", pr);
        println!("{}", pu);

        let s = "Hello World";
        println!("{:?}", s.as_bytes());
        let cipher = pu.encrypt(Plaintext::new(s));
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
        assert!(pu.authorize("file", &sign.to_str_radix(10)));
    }
}
