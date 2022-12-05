use std::{ops::RangeInclusive};
use aoc_runner_derive::aoc;
use anyhow::{Result, Context};

pub struct ElfPair {
    pub left: RangeInclusive<i32>,
    pub right: RangeInclusive<i32>
}

impl ElfPair {
    pub fn ranges_fully_overlap(&self) -> bool {
        let ElfPair { left, right } = self;
    
        [(left, right), (right, left)]
            .iter()
            .any(|(left, right)| left.contains(right.start()) && left.contains(right.end()))
    }    

    pub fn ranges_partially_overlap(&self) -> bool {
        let ElfPair { left, right } = self;

        [(left, right), (right, left)]
            .iter()
            .any(|(left, right)| left.contains(right.start()) || left.contains(right.end()))
    }
}

pub fn parse_range(range: &str) -> Result<RangeInclusive<i32>> {
    let (from, to) = range.split_once('-').context("Parsing error")?;
    let (from, to) = (from.parse::<i32>()?, to.parse::<i32>()?);

    Ok(from ..= to)
}

pub fn parse_line(line: &str) -> Result<ElfPair> {
    let (left, right) = line.split_once(',').context("Parsing error")?;
    let (left, right) = (parse_range(left)?, parse_range(right)?);
        
    let pair = ElfPair {
        left,
        right
    };

    Ok(pair)
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> Result<u32> {
    let mut count = 0;

    for line in input.lines() {
        let elf_pair = parse_line(line)?;

        if elf_pair.ranges_fully_overlap() { count += 1 }
    }

    Ok(count)
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> Result<u32> {    
    let mut count = 0;

    for line in input.lines() {
        let elf_pair = parse_line(line)?;

        if elf_pair.ranges_partially_overlap() { count += 1 }
    }

    Ok(count)
}