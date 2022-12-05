#![feature(iter_array_chunks)]
#![feature(array_try_from_fn)]
#![feature(result_flattening)]

use aoc_runner_derive::aoc_lib;

mod iterator_helpers;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

aoc_lib!{ year = 2022 }
