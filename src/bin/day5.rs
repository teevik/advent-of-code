#![feature(get_many_mut)]

use advent_of_code::execution_time;
use anyhow::{Context, Result};

fn before_instructions(line: &&str) -> bool {
    !line.starts_with(" 1   2")
}

type Crate = Vec<char>;
type Stacks = [Crate; 9];

pub fn parse_stacks(input: &str) -> Result<Stacks> {
    let mut stacks: [Vec<char>; 9] = Default::default();

    for line in input.lines().take_while(before_instructions) {
        for i in 0..9 {
            let char = line
                .chars()
                .nth((1 + (i * 4)) as usize)
                .context("Parsing error")?;

            if char != ' ' {
                stacks[i].insert(0, char);
            }
        }
    }

    Ok(stacks)
}

struct Instruction {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}

pub fn parse_instructions(input: &str) -> impl Iterator<Item = Result<Instruction>> + '_ {
    let instruction_lines = input.lines().skip_while(before_instructions).skip(2);

    instruction_lines.map(|instruction_line| {
        let mut instruction_parts = instruction_line.split(' ');

        let parse = |input: Option<&str>| {
            input
                .context("Parsing error")
                .and_then(|input| input.parse::<usize>().context("Parsing error"))
        };

        let amount = parse(instruction_parts.nth(1))?;
        let from = parse(instruction_parts.nth(1))? - 1;
        let to = parse(instruction_parts.nth(1))? - 1;

        Ok(Instruction { amount, from, to })
    })
}

pub fn solve_part1(input: &str) -> Result<String> {
    let mut stacks = parse_stacks(input)?;

    for instruction in parse_instructions(input) {
        let instruction = instruction?;

        for _ in 0..instruction.amount {
            let char = stacks[instruction.from]
                .pop()
                .context("No characters in crate")?;
            stacks[instruction.to].push(char)
        }
    }

    let result = stacks.iter().flat_map(|stack| stack.last()).collect();

    Ok(result)
}

pub fn solve_part2(input: &str) -> Result<String> {
    let mut stacks = parse_stacks(input)?;

    for instruction in parse_instructions(input) {
        let instruction = instruction?;

        let [from_vec, to_vec] = stacks.get_many_mut([instruction.from, instruction.to])?;

        let new_from_end = from_vec.len() - instruction.amount;
        let chars = &from_vec[new_from_end..];

        to_vec.extend_from_slice(chars);

        from_vec.truncate(new_from_end);
    }

    let result = stacks.iter().flat_map(|stack| stack.last()).collect();

    Ok(result)
}

pub fn main() -> Result<()> {
    let input = include_str!("../input/day5.txt");

    let part_1 = execution_time(|| solve_part1(input))?;
    dbg!(part_1);

    let part_2 = execution_time(|| solve_part2(input))?;
    dbg!(part_2);

    Ok(())
}
