#![feature(test)]

extern crate test;

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use test::Bencher;

/// A sample multi-threaded function.
fn execute() {
    type NumIter = usize;

    let num_iterations: NumIter = 1_000_000_000;

    let expected = num_iterations;
    let given: NumIter = (0..num_iterations).into_par_iter().map(|_| 1usize).sum();
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
