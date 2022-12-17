#![feature(get_many_mut)]

use advent_of_code::execution_time;
use glam::IVec2;
use hashbrown::HashSet;
use itertools::Itertools;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn as_ivec2(self) -> IVec2 {
        match self {
            Direction::Up => IVec2::new(0, -1),
            Direction::Right => IVec2::new(1, 0),
            Direction::Down => IVec2::new(0, 1),
            Direction::Left => IVec2::new(-1, 0),
        }
    }
}

struct RopeSimulation<const N: usize> {
    rope: [IVec2; N],
    visited_positions: HashSet<IVec2>,
}

impl<const N: usize> RopeSimulation<N> {
    pub fn new() -> Self {
        assert!(N >= 2);

        let center_position = IVec2::new(0, 0);

        let mut visited_positions = HashSet::new();
        visited_positions.insert(center_position);

        Self {
            rope: [center_position; N],
            visited_positions,
        }
    }

    pub fn simulate(&mut self, direction: Direction) {
        self.rope[0] += direction.as_ivec2();

        for (head_index, tail_index) in (0..N).tuple_windows() {
            let [head_position, tail_position] =
                self.rope.get_many_mut([head_index, tail_index]).unwrap();

            let diff = *head_position - *tail_position;

            let not_touching = diff.x.abs() > 1 || diff.y.abs() > 1;

            if not_touching {
                *tail_position += diff.signum();
                if tail_index == N - 1 {
                    self.visited_positions.insert(self.rope[N - 1]);
                }
            }
        }
    }

    pub fn amount_of_visited(&self) -> usize {
        self.visited_positions.len()
    }
}

pub fn solve<const N: usize>(input: &str) -> usize {
    let mut rope_simulation = RopeSimulation::<N>::new();

    for line in input.lines() {
        let (instruction, amount) = line.split_once(' ').unwrap();

        let direction = match instruction {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => unreachable!(),
        };
        let amount = amount.parse::<i32>().unwrap();

        for _ in 0..amount {
            rope_simulation.simulate(direction)
        }
    }

    rope_simulation.amount_of_visited()
}

pub fn solve_part1(input: &str) -> usize {
    solve::<2>(input)
}

pub fn solve_part2(input: &str) -> usize {
    solve::<10>(input)
}

pub fn main() {
    let input = include_str!("../input/day9.txt");

    let part_1 = execution_time(|| solve_part1(input));
    dbg!(part_1);

    let part_2 = execution_time(|| solve_part2(input));
    dbg!(part_2);
}
