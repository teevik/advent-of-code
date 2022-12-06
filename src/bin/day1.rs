use advent_of_code::execution_time;
use itertools::Itertools;

fn parse_input(input: &str) -> impl Iterator<Item = u32> + '_ {
    fn parse_calories_block(input: &str) -> u32 {
        input
            .split("\n")
            .map(|input| input.parse::<u32>().unwrap())
            .sum()
    }

    input.split("\n\n").map(parse_calories_block)
}

fn solve_part1(input: &str) -> u32 {
    let all_calories = parse_input(input);

    all_calories.max().unwrap()
}

fn solve_part2(input: &str) -> u32 {
    let all_calories = parse_input(input);

    all_calories.sorted().rev().take(3).sum()
}

pub fn main() {
    let input = include_str!("../input/day1.txt");

    let part_1 = execution_time(|| solve_part1(input));
    dbg!(part_1);

    let part_2 = execution_time(|| solve_part2(input));
    dbg!(part_2);
}
