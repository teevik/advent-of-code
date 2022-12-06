use advent_of_code::execution_time;
use anyhow::Result;
use anyhow::{Context, Error};
use byte_set::ByteSet;

pub fn find_index_with_distinct_characters(input: &str, amount: usize) -> Result<usize> {
    input
        .as_bytes()
        .windows(amount)
        .enumerate()
        .find_map(|(index, slice)| {
            let byte_set = ByteSet::from(slice);

            let len = byte_set.len();

            (len == amount).then_some(index + amount)
        })
        .context("Something went wrong")
}

pub fn solve_part1(input: &str) -> Result<usize> {
    find_index_with_distinct_characters(input, 4)
}

pub fn solve_part2(input: &str) -> Result<usize> {
    find_index_with_distinct_characters(input, 14)
}

pub fn main() -> Result<()> {
    let input = include_str!("../input/day6.txt");

    let part_1 = execution_time(|| solve_part1(input))?;
    dbg!(part_1);

    let part_2 = execution_time(|| solve_part2(input))?;
    dbg!(part_2);

    Ok(())
}
