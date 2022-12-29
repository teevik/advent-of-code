use advent_of_code::execution_time;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum Value {
    Integer(u8),
    List(Vec<Value>),
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        use Value::*;

        match (self, other) {
            (Integer(a), Integer(b)) => a.cmp(b),
            (List(a), List(b)) => a.cmp(b),
            (Integer(a), list) => List(vec![Integer(*a)]).cmp(list),
            (list, Integer(b)) => list.cmp(&List(vec![Integer(*b)])),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

mod parser {
    use crate::Value;
    use nom::branch::alt;
    use nom::character::complete::{char, u8};
    use nom::multi::separated_list0;
    use nom::sequence::delimited;
    use nom::Parser;
    use nom::{Finish, IResult};

    fn parse_value(input: &str) -> IResult<&str, Value> {
        alt((
            u8.map(Value::Integer),
            delimited(
                char('['),
                separated_list0(char(','), parse_value),
                char(']'),
            )
            .map(Value::List),
        ))(input)
    }

    pub fn parse(input: &str) -> impl Iterator<Item = (Value, Value)> + '_ {
        let pairs = input.split("\n\n");

        pairs.map(|pair| {
            let (a, b) = pair.split_once("\n").unwrap();

            let parse = |input| parse_value(input).finish().unwrap().1;

            let (a, b) = (parse(a), parse(b));

            (a, b)
        })
    }
}

fn solve_part1(input: &str) -> usize {
    let pairs = parser::parse(input);

    pairs
        .enumerate()
        .filter(|(_, (a, b))| a < b)
        .map(|(index, _)| index + 1)
        .sum()
}

fn solve_part2(input: &str) -> usize {
    use Value::*;

    let pairs = parser::parse(input);
    let packets = pairs.flat_map(|(a, b)| [a, b]);

    let divider_a = List(vec![List(vec![Integer(2)])]);
    let divider_b = List(vec![List(vec![Integer(6)])]);

    let mut all_packets = vec![divider_a.clone(), divider_b.clone()];
    all_packets.extend(packets);

    all_packets.sort_unstable();

    let result = all_packets.partition_point(|v| v <= &divider_a)
        * all_packets.partition_point(|v| v <= &divider_b);

    result
}

pub fn main() {
    let input = include_str!("../input/day13.txt");

    execution_time(|| dbg!(solve_part1(input)));

    execution_time(|| dbg!(solve_part2(input)));
}
