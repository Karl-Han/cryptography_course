extern crate num_bigint as bigint;
extern crate num_traits;
extern crate rand;

use bigint::*;
use core::ops::BitAnd;
use rand::distributions::{uniform::UniformSampler, Distribution};
use rand::RngCore;

struct PrimePair {
    p: BigUint,
    q: BigUint,
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
                println!("phead = {}", phead);
                println!("qhead = {}", qhead);
                println!("res= {}", res);
                break;
            }
        }

        let plow = BigUint::from(phead) << (psize - 32);
        let phigh = BigUint::from(1u32) << psize;
        println!("plow = {} with bits {}", plow, plow.bits());
        println!("phigh = {} with bits {}", phigh, phigh.bits());

        let sampler: UniformBigUint;
        if plow < phigh {
            sampler = UniformSampler::new(plow, phigh);
        } else {
            sampler = UniformSampler::new(phigh, plow);
        }
        let mut rng = rand::thread_rng();
        let mut psample: BigUint;

        loop {
            //sample = gen.sample(&mut rng);
            psample = sampler.sample(&mut rng);
            //println!("{} {}", sample, sample.bits());

            if psample.bits() == psize && miller_rabin(&psample) {
                println!("{} {}", psample, psample.bits());
                break;
            }
        }

        let qlow = BigUint::from(qhead) << (qsize - 32);
        let qhigh = BigUint::from(1u32) << qsize;
        println!("qlow = {} with bits {}", qlow, qlow.bits());
        println!("qhigh = {} with bits {}", qhigh, qhigh.bits());
        let sampler: UniformBigUint;
        let mut qsample: BigUint;
        if qlow < qhigh {
            sampler = UniformSampler::new(qlow, qhigh);
        } else {
            sampler = UniformSampler::new(qhigh, qlow);
        }

        loop {
            //sample = gen.sample(&mut rng);
            qsample = sampler.sample(&mut rng);
            //println!("{} {}", sample, sample.bits());

            if qsample.bits() == psize && miller_rabin(&qsample) {
                println!("{} {}", qsample, qsample.bits());
                break;
            }
        }

        PrimePair {
            p: psample,
            q: qsample,
        }
    }
    pub fn product(&self) -> BigUint {
        return self.p.clone() * self.q.clone();
    }
}

struct PrivateKey {
    p: BigUint,
    q: BigUint,
    e: BigUint,
    d: BigUint,
    n: BigUint,
}

struct PublicKey {
    e: BigUint,
    n: BigUint,
}

impl PrivateKey {
    // `size` is the size of modulus
    pub fn new(size: usize) {}
}

fn pass_miller_rabin(num: &BigUint, modulus: &BigUint) -> bool {
    let phi = modulus.clone() - 1u32;
    let mut odd: BigUint = phi.clone();

    while odd.clone().bitand(&num_traits::one()) == num_traits::one() {
        odd = odd / 2u32;
    }

    let mut base = num.modpow(&odd, &modulus);

    while phi >= odd {
        if base == num_traits::One::one() || base == num.clone() - 1u32 {
            return true;
        }
        base = base.modpow(&BigUint::from(2u32), &modulus);
        odd = odd * 2u32;
    }

    return false;
}

fn miller_rabin(num: &BigUint) -> bool {
    if num.bitand(BigUint::from(1u32)) != BigUint::from(1u32) {
        return false;
    }
    let mut rng = rand::thread_rng();

    let big_int_gen = UniformBigUint::new(num_traits::One::one(), num - 1u32);

    for _ in 0..50 {
        if !pass_miller_rabin(&big_int_gen.sample(&mut rng), &num) {
            return false;
        }
    }
    return true;
}

fn generate_prime(size: usize) -> BigUint {
    let mut rng = rand::thread_rng();

    let mut sample: BigUint;
    let gen = RandomBits::new(size);

    let low = BigUint::from(1u32) << (size - 1);
    let high = BigUint::from(1u32) << size;
    let sampler: UniformBigUint = UniformSampler::new(low, high);

    loop {
        //sample = gen.sample(&mut rng);
        sample = sampler.sample(&mut rng);
        //println!("{} {}", sample, sample.bits());

        if sample.bits() == size && miller_rabin(&sample) {
            println!("{}", sample);
            return sample;
        }
    }
}

fn main() {
    //let mut rng = rand::thread_rng();

    // let low: RandomBits = RandomBits::new(1023);
    //let num: BigUint = Distribution::sample(&low, &mut rng);
    //println!("{}", num);

    //let mut n: BigUint;

    //loop {
    //    let p = generate_prime(512);
    //    let q = generate_prime(512);

    //    println!("p = {}", p);
    //    println!("q = {}", q);

    //    n = p * q;

    //    if n.bits() == 1024 {
    //        break;
    //    }
    //}

    //println!("n = {}", n);

    //let num: BigUint = UniformBigUint::new(low, high);

    //println!("{}", num.bits());

    let pair = PrimePair::new(512, 512);

    let n = pair.product();
    println!("{} with {} bits", n, n.bits());
}
