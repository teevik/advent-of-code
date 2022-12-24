use advent_of_code::execution_time;
use anyhow::Result;
use std::mem;

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    pub fn run(self, old: u64) -> u64 {
        match self {
            Operation::Add(other) => old + other,
            Operation::Multiply(other) => old * other,
            Operation::Square => old * old,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rule {
    pub divisor: u64,
    pub success: usize,
    pub fail: usize,
}

impl Rule {
    pub fn test(self, item: u64) -> usize {
        if item % self.divisor == 0 {
            self.success
        } else {
            self.fail
        }
    }
}

#[derive(Debug)]
pub struct Monkey {
    pub id: usize,
    pub items: Vec<u64>,
    pub operation: Operation,
    pub rule: Rule,
    pub inspections: u64,
}

mod parser {
    use super::*;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{multispace1, newline, space1, u64};
    use nom::combinator::{map, value};
    use nom::multi::separated_list1;
    use nom::sequence::{delimited, preceded, terminated};
    use nom::{Finish, IResult};

    fn id(s: &str) -> IResult<&str, usize> {
        map(delimited(tag("Monkey "), u64, tag(":")), |n| n as usize)(s)
    }

    fn items(s: &str) -> IResult<&str, Vec<u64>> {
        let prefix = preceded(space1, tag("Starting items: "));
        let list = separated_list1(tag(", "), u64);
        map(preceded(prefix, list), Vec::from)(s)
    }

    fn add_operation(s: &str) -> IResult<&str, Operation> {
        map(preceded(tag("+ "), u64), Operation::Add)(s)
    }

    fn multiply_operation(s: &str) -> IResult<&str, Operation> {
        map(preceded(tag("* "), u64), Operation::Multiply)(s)
    }

    fn square_operation(s: &str) -> IResult<&str, Operation> {
        value(Operation::Square, tag("* old"))(s)
    }

    fn operation_prefix(s: &str) -> IResult<&str, &str> {
        preceded(space1, tag("Operation: new = old "))(s)
    }

    fn operation(s: &str) -> IResult<&str, Operation> {
        let add = preceded(operation_prefix, add_operation);
        let multiply = preceded(operation_prefix, multiply_operation);
        let square = preceded(operation_prefix, square_operation);

        alt((add, multiply, square))(s)
    }

    fn test_rule(input: &str) -> IResult<&str, Rule> {
        let (input, divisor) = preceded(space1, preceded(tag("Test: divisible by "), u64))(input)?;

        let (input, success) =
            preceded(multispace1, preceded(tag("If true: throw to monkey "), u64))(input)?;

        let (input, fail) = preceded(
            multispace1,
            preceded(tag("If false: throw to monkey "), u64),
        )(input)?;

        let rule = Rule {
            divisor,
            success: success as usize,
            fail: fail as usize,
        };

        Ok((input, rule))
    }

    fn monkey(input: &str) -> IResult<&str, Monkey> {
        let (input, id) = terminated(id, newline)(input)?;
        let (input, items) = terminated(items, newline)(input)?;
        let (input, operation) = terminated(operation, newline)(input)?;
        let (input, rule) = test_rule(input)?;

        let monkey = Monkey {
            id,
            items,
            operation,
            rule,
            inspections: 0,
        };

        Ok((input, monkey))
    }

    pub fn parse(input: &'static str) -> Result<Vec<Monkey>> {
        let result = separated_list1(tag("\n\n"), monkey)(input);

        let (_, monkeys) = result.finish()?;

        Ok(monkeys)
    }
}

fn solve_part1(input: &'static str) -> u64 {
    let mut monkeys = parser::parse(input).unwrap();

    for _round in 0..20 {
        for monkey_id in 0..monkeys.len() {
            let items = mem::take(&mut monkeys[monkey_id].items);

            for mut item in items {
                monkeys[monkey_id].inspections += 1;
                item = monkeys[monkey_id].operation.run(item);

                item /= 3;

                let monkey_throw_to = monkeys[monkey_id].rule.test(item);
                monkeys[monkey_throw_to].items.push(item);
            }
        }
    }

    monkeys.sort_unstable_by_key(|monkey| monkey.inspections);

    monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .rev()
        .take(2)
        .product()
}

fn solve_part2(input: &'static str) -> u64 {
    let mut monkeys = parser::parse(input).unwrap();

    let limit = monkeys
        .iter()
        .map(|monkey| monkey.rule.divisor)
        .product::<u64>();

    for _round in 0..10000 {
        for monkey_id in 0..monkeys.len() {
            let items = mem::take(&mut monkeys[monkey_id].items);

            for mut item in items {
                monkeys[monkey_id].inspections += 1;
                item = monkeys[monkey_id].operation.run(item);

                item %= limit;

                let monkey_throw_to = monkeys[monkey_id].rule.test(item);
                monkeys[monkey_throw_to].items.push(item);
            }
        }
    }

    monkeys.sort_unstable_by_key(|monkey| monkey.inspections);

    monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .rev()
        .take(2)
        .product()
}

pub fn main() {
    let input = include_str!("../input/day11.txt");

    execution_time(|| dbg!(solve_part1(input)));

    execution_time(|| dbg!(solve_part2(input)));
}
