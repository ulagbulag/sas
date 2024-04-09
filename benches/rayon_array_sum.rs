#![feature(test)]

extern crate test;

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use test::Bencher;

type NumIter = usize;

const NUM_ITERATIONS: NumIter = 1_000_000;

/// A sample multi-threaded function.
fn execute(iter: &[NumIter]) {
    let expected = NUM_ITERATIONS * (NUM_ITERATIONS - 1) / 2;
    let given: NumIter = iter.into_par_iter().sum();
    assert_eq!(expected, given);
}

#[bench]
fn bench_rayon_array_sum_with_rayon(b: &mut Bencher) {
    ::sas::init();

    let iterations: Vec<NumIter> = (0..NUM_ITERATIONS).collect();

    b.iter(|| execute(&iterations))
}

#[bench]
fn bench_rayon_array_sum_without_rayon(b: &mut Bencher) {
    let iterations: Vec<NumIter> = (0..NUM_ITERATIONS).collect();

    b.iter(|| execute(&iterations))
}
