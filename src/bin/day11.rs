use advent_of_code::execution_time;
use itertools::Itertools;

enum OperationVariable {
    Old,
    Constant(i32),
}

enum Operation {
    Add,
    Multiply,
}

struct Monkey {
    items: Vec<i32>,
}

fn solve_part1(input: &str) -> () {
    for monkey_chunk in input.split("\n\n") {
        let mut lines = monkey_chunk.lines();

        lines.next();

        let worry_levels = {
            let parts = lines.next().unwrap().split(' ').skip(4);

            parts.map(|starting_item| starting_item.parse::<i32>())
        };

        let operation = {
            let mut parts = lines.next().unwrap().split(' ').skip(6);

            let operation = {
                let char = parts.next().unwrap();

                match char {
                    "+" => Operation::Add,
                    "*" => Operation::Multiply,
                    _ => unreachable!(),
                }
            };

            let left_side = OperationVariable::Old;

            let right_side = {
                let right_side = parts.next().unwrap();

                match right_side {
                    "old" => OperationVariable::Old,
                    _ => OperationVariable::Constant(right_side.parse().unwrap()),
                }
            };
        };

        let test_divisible_by = {
            let divisible_by = lines.next().unwrap().split(' ').nth(5).unwrap();

            divisible_by.parse::<i32>().unwrap()
        };

        let test_divisible_by = {
            let divisible_by = lines.next().unwrap().split(' ').nth(5).unwrap();

            divisible_by.parse::<i32>().unwrap()
        };
    }

    ()
}

pub fn main() {
    let input = include_str!("../input/day11.txt");

    execution_time(|| dbg!(solve_part1(input)));
}
