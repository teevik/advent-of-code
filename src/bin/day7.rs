use advent_of_code::execution_time;
use anyhow::Result;

pub fn solve_part1(input: &str) -> Result<()> {
    Ok(())
}

pub fn main() -> Result<()> {
    let input = include_str!("../input/day7.txt");

    let part_1 = execution_time(|| solve_part1(input))?;
    dbg!(part_1);

    // let part_2 = execution_time(|| solve_part2(input))?;
    // dbg!(part_2);

    Ok(())
}
