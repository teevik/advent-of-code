#![feature(array_try_from_fn)]
#![feature(result_flattening)]
#![feature(fn_traits)]

use itertools::any;
use std::time::Instant;
use vek::Vec2;

pub mod iterator_helpers;

pub fn execution_time<T>(runner: impl Fn() -> T) -> T {
    let now = Instant::now();

    let output = runner();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    output
}
