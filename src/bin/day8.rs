use advent_of_code::execution_time;
use anyhow::Result;

pub fn solve_part1(input: &str) -> Result<()> {
    let grid = input.lines().collect::<Vec<_>>();

    let mut count = 0;

    for (y, &line) in grid.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let higher = |other_char| char > other_char;
            let pog = line.chars().take(x).all(higher)
                || line.chars().skip(x + 1).all(higher)
                || grid
                    .iter()
                    .take(y)
                    .all(|&line| higher(line.chars().nth(x).unwrap()))
                || grid
                    .iter()
                    .skip(y + 1)
                    .all(|&line| higher(line.chars().nth(x).unwrap()));

            // let dam = ;
            if pog {
                count += 1
            };

            // count += 1;
        }
    }

    dbg!(count);

    Ok(())
}

pub fn main() -> Result<()> {
    let input = include_str!("../input/day8.txt");

    let part_1 = execution_time(|| solve_part1(input))?;
    dbg!(part_1);

    Ok(())
}
