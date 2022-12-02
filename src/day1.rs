use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    fn parse_calories_block(input: &str) -> u32 {
        input
            .split("\n")
            .map(|input| input.parse::<u32>().unwrap())
            .sum()
    }

    input.split("\n\n")
        .map(parse_calories_block)
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(all_calories: &[u32]) -> u32 {
    *all_calories.iter()
        .max()
        .unwrap()

}

#[aoc(day1, part2)]
pub fn solve_part2(all_calories: &[u32]) -> u32 {
    all_calories.iter()
        .sorted()
        .rev()
        .take(3)
        .sum()
}
