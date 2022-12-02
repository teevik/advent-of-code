use aoc_runner_derive::{aoc};

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
            GameResult::Win => 6
        }
    }
}

#[derive(Clone, Copy)]
pub enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    pub fn calculate_result(self, other_move: Move) -> GameResult {
        match (other_move, self) {
            (Move::Rock, Move::Scissors) => GameResult::Loss,
            (Move::Paper, Move::Rock) => GameResult::Loss,
            (Move::Scissors, Move::Paper) => GameResult::Loss,
            (Move::Rock, Move::Rock) => GameResult::Draw,
            (Move::Paper, Move::Paper) => GameResult::Draw,
            (Move::Scissors, Move::Scissors) => GameResult::Draw,
            (Move::Rock, Move::Paper) => GameResult::Win,
            (Move::Paper, Move::Scissors) => GameResult::Win,
            (Move::Scissors, Move::Rock) => GameResult::Win,
        }
    }

    pub fn from_result(result: GameResult, other_move: Move) -> Move {
        match (other_move, result) {
            (Move::Rock, GameResult::Loss) => Move::Scissors,
            (Move::Paper, GameResult::Loss) => Move::Rock,
            (Move::Scissors, GameResult::Loss) => Move::Paper,
            (Move::Rock, GameResult::Draw) => Move::Rock,
            (Move::Paper, GameResult::Draw) => Move::Paper,
            (Move::Scissors, GameResult::Draw) => Move::Scissors,
            (Move::Rock, GameResult::Win) => Move::Paper,
            (Move::Paper, GameResult::Win) => Move::Scissors,
            (Move::Scissors, GameResult::Win) => Move::Rock,
        }
    }

    pub fn score(self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3
        }
    }
}

pub struct Part1Game {
    elf_move: Move,
    player_move: Move
}

impl Part1Game {
    pub fn score(&self) -> u32 {
        let game_result = self.player_move.calculate_result(self.elf_move);

        self.player_move.score() + game_result.score()
    }
}

pub fn parse_part1(input: &str) -> Vec<Part1Game> {
    fn parse_line(input: &str) -> Part1Game {
        let mut chars = input.split(" ");

        let elf_move = match chars.next().unwrap() {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!()
        };

        let player_move = match chars.next().unwrap() {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!()
        };

        Part1Game {
            elf_move,
            player_move
        }
    }

    input.split("\n")
        .map(parse_line)
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let games = parse_part1(input);

    games.iter()
        .map(Part1Game::score)
        .sum()
}

pub struct Part2Game {
    elf_move: Move,
    game_result: GameResult
}

impl Part2Game {
    pub fn score(&self) -> u32 {
        let player_move = Move::from_result(self.game_result, self.elf_move);

        self.game_result.score() + player_move.score()
    }
}

pub fn parse_part2(input: &str) -> Vec<Part2Game> {
    fn parse_line(input: &str) -> Part2Game {
        let mut chars = input.split(" ");

        let elf_move = match chars.next().unwrap() {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!()
        };

        let game_result = match chars.next().unwrap() {
            "X" => GameResult::Loss,
            "Y" => GameResult::Draw,
            "Z" => GameResult::Win,
            _ => panic!()
        };

        Part2Game {
            elf_move,
            game_result
        }
    }

    input.split("\n")
        .map(parse_line)
        .collect()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let games = parse_part2(input);

    games.iter()
        .map(Part2Game::score)
        .sum()
}
