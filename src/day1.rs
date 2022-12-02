use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

pub struct Elf {
    calories: u32
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Elf> {
    fn parse_calories_block(input: &str) -> Elf {
        let calories = input
            .split("\n")
            .map(|input| input.parse::<u32>().unwrap())
            .sum();

        Elf {
            calories
        }
    }

    input.split("\n\n")
        .map(parse_calories_block)
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(elves: &[Elf]) -> u32 {
    elves.iter()
        .map(|elf| elf.calories)
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(elves: &[Elf]) -> u32 {
    elves.iter()
        .map(|elf| elf.calories)
        .sorted()
        .rev()
        .take(3)
        .sum()
}
