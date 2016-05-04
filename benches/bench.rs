#![feature(test)]
extern crate rand;
extern crate rand_mersenne_twister as mersenne_twister;
extern crate test;

const BENCH_N: u64 = 1000;

use std::mem::size_of;
use rand::{OsRng, Rng};
use test::{black_box, Bencher};
use mersenne_twister::{MTRng32, MTRng64};

#[bench]
fn bench_mt32(b: &mut Bencher) {
    let mut rng: MTRng32 = OsRng::new().unwrap().gen();
    b.iter(|| {
        for _ in 0..BENCH_N {
            black_box(rng.gen::<usize>());
        }
    });
    b.bytes = BENCH_N * size_of::<usize>() as u64;
}

#[bench]
fn bench_mt64(b: &mut Bencher) {
    let mut rng: MTRng64 = OsRng::new().unwrap().gen();
    b.iter(|| {
        for _ in 0..BENCH_N {
            black_box(rng.gen::<usize>());
        }
    });
    b.bytes = BENCH_N * size_of::<usize>() as u64;
}
