extern crate num_bigint;
extern crate rand;

use core::ops::BitAnd;
use num_bigint::{BigInt, UniformBigInt};
use rand::distributions::uniform::UniformSampler;

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
