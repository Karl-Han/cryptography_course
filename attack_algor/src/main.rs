use num_bigint::{BigInt, BigUint, ParseBigIntError};
use num_traits::cast::ToPrimitive;
use std::str::FromStr;

struct ContinuedFractionStream {
    numberator: BigUint,
    denominator: BigUint,
    quotients: Vec<u128>,
}

impl ContinuedFractionStream {
    pub fn new(num: BigUint, deno: BigUint) -> ContinuedFractionStream {
        let f = num.clone() / deno.clone();
        let f = f.to_u128().unwrap();
        let numberator = num.clone() - f.clone() * deno.clone();
        let denominator = deno;
        //println!("f = {}", f);

        let v = vec![f];
        ContinuedFractionStream {
            numberator,
            denominator,
            quotients: v,
        }
    }
    pub fn next(&mut self) -> Option<u128> {
        if self.numberator == BigUint::from(0u32) {
            return None;
        }
        let q = self.denominator.clone() / self.numberator.clone();
        let q = q.to_u128().unwrap();
        let r = self.denominator.clone() - q.clone() * self.numberator.clone();

        self.denominator = self.numberator.clone();
        self.numberator = r;
        self.quotients.push(q);

        return Some(q);
    }
    pub fn quotients(&self) -> &Vec<u128> {
        &self.quotients
    }
    pub fn gen_all(&mut self) -> &Self {
        while let Some(_) = self.next() {}
        self
    }
}

pub fn expand_to_i(slice: &[u128], i: usize) -> f64 {
    return slice[0] as f64 + expansion(&slice[1..i + 1]);
}

pub fn expansion(slice: &[u128]) -> f64 {
    if slice.len() == 0 {
        return 0f64;
    }

    let res: f64 = slice[0] as f64 + expansion(&slice[1..]);
    //println!("res = {}", 1f64 / res);

    return 1f64 / res;
}

// For p and q that is really close
// Described in https://lixingcong.github.io/2016/04/03/Cryptography-I-week-6/
#[allow(dead_code)]
fn p_q_close(n: &BigInt) -> (BigInt, BigInt) {
    let sqrt_n = n.clone().sqrt();
    println!("sqrt_n = {}", sqrt_n);
    let a = sqrt_n + 1u32;

    let x = (a.clone() * a.clone() - n).sqrt();
    println!("a = {}\nx = {}", a, x);

    let p = a.clone() - x.clone();
    let q = a.clone() + x.clone();
    println!("p = {}\nq = {}", p, q);
    (p, q)
}

fn main() -> Result<(), ParseBigIntError> {
    //let s = "8419248954524000439721779172023134688983838205866625782151550834434276874684863239544369195264071670152656061813873751842115416791829324879655667191724512456544905595733991629887800889255133717212624547817690492648616532902257249552981800714896543008295153051040335475732125114592095784407296265046992475467";
    //let n = BigInt::from_str(s).expect("Error when parse from str");
    //let (p, q) = p_q_close(&n);
    //assert_eq!(p * q, n);
    //println!("p = {}\nq = {}\nn = {}", p, q, p.clone() * q.clone());
    let n = BigUint::from_str("205320043521075746592613")?;
    let e = BigUint::from_str("70760135995620281241019")?;

    let mut ps = ContinuedFractionStream::new(e, n);
    ps.gen_all();
    println!("{:?}", ps.quotients());

    //let length: usize = ps.quotients().len();
    //for i in 0..length {
    //    println!("{} th is {}", i, expand_to_i(&ps.quotients(), i));
    //}

    Ok(())
}
