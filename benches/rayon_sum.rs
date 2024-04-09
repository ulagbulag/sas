#![feature(test)]

extern crate test;

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use test::Bencher;

type NumIter = usize;

const NUM_ITERATIONS: NumIter = 1_000_000;

/// A sample multi-threaded function.
fn execute() {
    let expected = NUM_ITERATIONS * (NUM_ITERATIONS - 1) / 2;
    let given: NumIter = (0..NUM_ITERATIONS).into_par_iter().sum();
    assert_eq!(expected, given);
}

#[bench]
fn bench_rayon_sum_with_rayon(b: &mut Bencher) {
    ::sas::init();
    b.iter(execute)
}

#[bench]
fn bench_rayon_sum_without_rayon(b: &mut Bencher) {
    b.iter(execute)
}
