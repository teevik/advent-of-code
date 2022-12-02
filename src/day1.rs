use aoc_runner_derive::aoc;
use itertools::Itertools;

pub fn parse_input(input: &str) -> impl Iterator<Item = u32> + '_ {
    fn parse_calories_block(input: &str) -> u32 {
        input
            .split("\n")
            .map(|input| input.parse::<u32>().unwrap())
            .sum()
    }

    input.split("\n\n")
        .map(parse_calories_block)
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let all_calories = parse_input(input);

    all_calories
        .max()
        .unwrap()

}

#[aoc(day1, part2, Sort)]
pub fn solve_part2_sort(input: &str) -> u32 {
    let all_calories = parse_input(input);

    all_calories
        .sorted()
        .rev()
        .take(3)
        .sum()
}

#[aoc(day1, part2, Fold)]
pub fn solve_part2_fold(input: &str) -> u32 {
    let all_calories = parse_input(input);

    let three_biggest_calories = all_calories
        .fold([0, 0, 0], |mut acc, calories| {
            for existing_max in &mut acc {
                if calories > *existing_max {
                    *existing_max = calories;
                    break;
                }
            }

            acc
        });

    three_biggest_calories
        .iter()
        .sum()
}
