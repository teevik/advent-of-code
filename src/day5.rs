use anyhow::{Context, Result};
use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> Result<String> {    
    let mut crates: [Vec<char>; 9] = Default::default();

    let before_instructions = |line: &&str| !line.starts_with(" 1   2");

    for line in input.lines().take_while(before_instructions) {
        for i in 0..9 {
            let char = line.chars().nth((1 + (i * 4)) as usize).context("Parsing error")?;
            
            if char != ' ' {
                crates[i].insert(0, char);
            }
        }
    }

    for instruction_line in input.lines().skip_while(before_instructions).skip(2) {
        let mut instruction_parts = instruction_line.split(' ');

        let parse = |input: Option<&str>| {
            input.context("Parsing error")
                .and_then(|input| input.parse::<usize>().context("Parsing error"))
        };

        let amount = parse(instruction_parts.nth(1))?;
        let from = parse(instruction_parts.nth(1))? - 1;
        let to = parse(instruction_parts.nth(1))? - 1;


        for _ in 0..amount {
            let char = crates[from].pop().context("No characters in crate")?;
            crates[to].push(char)
        }
    }

    let a = crates.iter().flat_map(|a| a.last()).collect();

    Ok(a)
    }