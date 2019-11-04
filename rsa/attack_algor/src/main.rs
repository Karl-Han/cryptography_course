use fraction::Ratio;
use num_bigint::{BigInt, BigUint, ParseBigIntError};
use num_traits::{
    identities::{One, Zero},
    sign::Signed,
};
use std::str::FromStr;

struct ContinuedRatioStream {
    numberator: BigUint,
    denominator: BigUint,
    quotients: Vec<BigUint>,
}

impl ContinuedRatioStream {
    pub fn new(num: &BigUint, deno: &BigUint) -> ContinuedRatioStream {
        let f = num.clone() / deno.clone();
        let numberator = num.clone() - f.clone() * deno.clone();
        let denominator = deno.clone();
        //println!("f = {}", f);

        let v = vec![f];
        ContinuedRatioStream {
            numberator,
            denominator,
            quotients: v,
        }
    }
    pub fn next(&mut self) -> Option<BigUint> {
        if self.numberator == BigUint::from(0u32) {
            return None;
        }
        let q = self.denominator.clone() / self.numberator.clone();
        let r = self.denominator.clone() - q.clone() * self.numberator.clone();

        self.denominator = self.numberator.clone();
        self.numberator = r;
        self.quotients.push(q.clone());

        return Some(q);
    }
    pub fn quotients(&self) -> &Vec<BigUint> {
        &self.quotients
    }
    pub fn gen_all(&mut self) -> &Self {
        while let Some(_) = self.next() {}
        self
    }
}

pub fn expand_to_i(slice: &[BigUint], i: usize) -> (BigUint, BigUint) {
    let res = Ratio::new(slice[0].clone(), BigUint::from(1u64)) + expansion(&slice[1..i]);
    return (res.numer().clone(), res.denom().clone());
}

pub fn expansion(slice: &[BigUint]) -> Ratio<BigUint> {
    if slice.len() == 0 {
        return Ratio::zero();
    }

    //let res: f64 = slice[0] as f64 + expansion(&slice[1..]);
    //println!("res = {}", 1f64 / res);

    return (Ratio::from(slice[0].clone()) + Ratio::from(expansion(&slice[1..]))).recip();
}

pub fn solve_quadratic(a: &BigInt, b: &BigInt, c: &BigInt) -> (BigInt, BigInt) {
    //println!("a = {}", a);
    //println!("b = {}", b);
    //println!("c = {}", c);
    let b4ac: BigInt = b.clone() * b.clone() - BigInt::from(4i32) * a.clone() * c.clone();
    //println!("b4ac = {}", b4ac);
    //println!("-b = {}", -b);
    let temp_p: BigInt = (-BigInt::from(b.clone()) + b4ac.clone().sqrt()) / 2;
    //println!("temp_p = {}", temp_p);
    let temp_q: BigInt = (-BigInt::from(b.clone()) - b4ac.sqrt()) / 2;
    //println!("temp_q = {}", temp_q);
    return (temp_p, temp_q);
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

    let mut ps = ContinuedRatioStream::new(&e, &n);
    ps.gen_all();
    //println!("{:?}", ps.quotients());

    let length: usize = ps.quotients().len();
    let mut p: BigInt = BigInt::zero();
    let mut q: BigInt = BigInt::zero();
    for i in 2..length {
        let radio = expand_to_i(&ps.quotients(), i);
        let k = radio.0;
        let d = radio.1;
        //println!("k = {}, d = {}", k, d);
        let phi_n = BigInt::from(e.clone() * d.clone() - BigUint::one()) / BigInt::from(k);
        //println!("phi_n = {}", phi_n);
        let a = BigInt::one();
        let b = BigInt::from(BigInt::from(n.clone()) - phi_n + BigInt::one());
        //println!("b = {}", b);
        let c = BigInt::from(n.clone());
        //println!("c = {}", c);
        let pair = solve_quadratic(&a, &b, &c);
        p = pair.0;
        q = pair.1;
        let res = p.clone() * q.clone();
        //println!("p * q == {}", res);
        if res == BigInt::from(n.clone()) {
            p = p.abs();
            q = q.abs();
            println!("p = {}\nq = {}", p, q);
            break;
        }
    }

    Ok(())
}
