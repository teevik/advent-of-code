use advent_of_code::execution_time;
use anyhow::{Context, Result};
use itertools::Itertools;

#[derive(Clone, Copy)]
pub enum GameResult {
    Loss,
    Draw,
    Win,
}

impl GameResult {
    pub fn score(self) -> u32 {
        match self {
            GameResult::Loss => 0,
            GameResult::Draw => 3,
            GameResult::Win => 6,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn wins_against(self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn loses_to(self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    pub fn calculate_result(self, other_move: Move) -> GameResult {
        if self == other_move {
            GameResult::Draw
        } else if self.loses_to() == other_move {
            GameResult::Loss
        } else {
            GameResult::Win
        }
    }

    pub fn from_result(result: GameResult, other_move: Move) -> Move {
        match result {
            GameResult::Loss => other_move.wins_against(),
            GameResult::Draw => other_move,
            GameResult::Win => other_move.loses_to(),
        }
    }

    pub fn score(self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

pub struct Part1Game {
    elf_move: Move,
    player_move: Move,
}

impl Part1Game {
    pub fn score(&self) -> u32 {
        let game_result = self.player_move.calculate_result(self.elf_move);

        self.player_move.score() + game_result.score()
    }
}

pub fn parse_part1(input: &str) -> impl Iterator<Item = Result<Part1Game>> + '_ {
    fn parse_line(input: &str) -> Result<Part1Game> {
        let (char1, char2) = input.split_once(' ').context("Parsing error")?;

        let elf_move = match char1 {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!(),
        };

        let player_move = match char2 {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!(),
        };

        Ok(Part1Game {
            elf_move,
            player_move,
        })
    }

    input.split("\n").map(parse_line)
}

pub fn solve_part1(input: &str) -> Result<u32> {
    let games = parse_part1(input);

    games.map_ok(|game| game.score()).sum()
}

pub struct Part2Game {
    elf_move: Move,
    game_result: GameResult,
}

impl Part2Game {
    pub fn score(&self) -> u32 {
        let player_move = Move::from_result(self.game_result, self.elf_move);

        self.game_result.score() + player_move.score()
    }
}

pub fn parse_part2(input: &str) -> impl Iterator<Item = Result<Part2Game>> + '_ {
    fn parse_line(input: &str) -> Result<Part2Game> {
        let (char1, char2) = input.split_once(' ').context("Parsing error")?;

        let elf_move = match char1 {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!(),
        };

        let game_result = match char2 {
            "X" => GameResult::Loss,
            "Y" => GameResult::Draw,
            "Z" => GameResult::Win,
            _ => panic!(),
        };

        Ok(Part2Game {
            elf_move,
            game_result,
        })
    }

    input.split("\n").map(parse_line)
}

pub fn solve_part2(input: &str) -> Result<u32> {
    let games = parse_part2(input);

    games.map_ok(|game| game.score()).sum()
}

pub fn main() -> Result<()> {
    let input = include_str!("../input/day2.txt");

    let part_1 = execution_time(|| solve_part1(input))?;
    dbg!(part_1);

    let part_2 = execution_time(|| solve_part2(input))?;
    dbg!(part_2);

    Ok(())
}
