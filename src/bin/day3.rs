#![feature(iter_array_chunks)]

use advent_of_code::execution_time;
use anyhow::{Context, Result};
use byte_set::ByteSet;
use itertools::Itertools;

fn get_score(character: u8) -> u32 {
    if character <= 90 {
        character as u32 - 38
    } else {
        character as u32 - 96
    }
}

fn split_string_at_middle(input: &str) -> [&str; 2] {
    let (a, b) = input.split_at(input.len() / 2);

    [a, b]
}

fn intersection(byte_sets: impl IntoIterator<Item = impl Into<ByteSet>>) -> ByteSet {
    byte_sets
        .into_iter()
        .map(|byte_set| byte_set.into())
        .reduce(|a, b| a.intersection(b))
        .unwrap_or_default()
}

pub fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| split_string_at_middle(line))
        .flat_map(|parts| intersection(parts))
        .map(|character| get_score(character))
        .sum()
}

pub fn solve_part2(input: &str) -> Result<u32> {
    input
        .lines()
        .array_chunks::<3>()
        .map(|chunk| intersection(chunk).first().context("Parsing error"))
        .map_ok(|character| get_score(character))
        .sum()
}

pub fn main() -> Result<()> {
    let input = include_str!("../input/day3.txt");

    let part_1 = execution_time(|| solve_part1(input));
    dbg!(part_1);

    let part_2 = execution_time(|| solve_part2(input))?;
    dbg!(part_2);

    Ok(())
}
