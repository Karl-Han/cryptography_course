extern crate test;

use test::Bencher;
use crate::lib::*;

#[bench]
fn bench_keccak_256_input_4096_bytes(b: &mut Bencher) {
    let data = [254u8; 4096];
    b.bytes = data.len() as u64;

    b.iter(|| {
        let mut res: [u8; 32] = [0; 32];
        let mut keccak = Keccakf::new_v256();
        keccak.hash(&data);
        keccak.finalize(&mut res);
    });
}

#[bench]
fn keccakf_u64(b: &mut Bencher) {
    const WORDS: usize = 25;
    b.bytes = (WORDS * 8) as u64;

    b.iter(|| {
        let mut buf = Buffer::new();
        buf.keccak(24);
    });
}

#[bench]
fn bench_keccak256(b: &mut Bencher) {
    let data = [0u8; 32];
    b.bytes = data.len() as u64;

    b.iter(|| {
        let mut res: [u8; 32] = [0; 32];
        let mut keccak = Keccakf::new_v256();
        keccak.hash(&data);
        keccak.finalize(&mut res);
    });
}

