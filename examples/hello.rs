use std::time::Instant;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

/// A sample multi-threaded function.
fn my_heavy_work() {
    type NumIter = usize;

    let num_iterations: NumIter = 1_000_000_000;

    let expected = num_iterations;
    let given: NumIter = (0..num_iterations).into_par_iter().map(|_| 1usize).sum();
    assert_eq!(expected, given);
}

fn main() {
    // That's end!
    ::sas::init();

    // ... your heavy works will be here
    let begin_time = Instant::now();
    my_heavy_work();

    let elapsed_time = begin_time.elapsed();
    println!("Elapsed time: {elapsed_time:?}");
}
