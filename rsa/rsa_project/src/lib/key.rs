extern crate num_bigint;
extern crate rand;
extern crate sha3;

use super::{
    cipher_plain::{Cipher, Plaintext},
    primality_test::{egcd, miller_rabin, swap},
};
use num_bigint::{BigInt, Sign, UniformBigInt};
use rand::{distributions::uniform::UniformSampler, RngCore};
use sha3::{Digest, Sha3_256};
use std::fmt;
use std::fs::File;
use std::io;
use std::str::FromStr;

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

#[derive(Clone)]
pub struct PrimePair {
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

impl fmt::Display for PrimePair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "p : {}\nq : {}", self.p, self.q)
    }
}

#[derive(Clone)]
pub struct PrivateKey {
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
    pub fn d(&self) -> BigInt {
        return self.d.clone();
    }
    pub fn n(&self) -> BigInt {
        return self.n.clone();
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

impl fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PrivateKey: [{}\nd : {}\ne : {}\nn : {}]",
            self.p_q_pair, self.d, self.e, self.n
        )
    }
}

#[derive(Debug)]
pub struct PublicKey {
    e: BigInt,
    n: BigInt,
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
    pub fn e(&self) -> BigInt {
        return self.e.clone();
    }
    pub fn n(&self) -> BigInt {
        return self.n.clone();
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PublicKey:[e = {}\nn = {}]", self.e, self.n)
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
