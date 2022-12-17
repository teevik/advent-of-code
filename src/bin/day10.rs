use advent_of_code::execution_time;
use itertools::Itertools;

struct Simulation {
    register: i32,
    cycle: i32,
}
fn solve(input: &str, mut run_cycle: impl FnMut(&Simulation) -> ()) {
    let mut simulation = Simulation {
        register: 1,
        cycle: 1,
    };

    for line in input.lines() {
        let mut words = line.split_whitespace();

        let instruction = words.next().unwrap();

        match instruction {
            "noop" => {
                run_cycle(&simulation);
                simulation.cycle += 1;
            }
            "addx" => {
                run_cycle(&simulation);
                simulation.cycle += 1;

                run_cycle(&simulation);
                simulation.cycle += 1;

                let amount: i32 = words.next().unwrap().parse().unwrap();
                simulation.register += amount;
            }
            _ => unreachable!(),
        }
    }
}

fn solve_part1(input: &str) -> i32 {
    let mut total = 0;

    solve(input, |simulation| {
        if simulation.cycle % 40 == 20 {
            total += simulation.cycle * simulation.register;
        }
    });

    total
}

fn solve_part2(input: &str) -> String {
    let mut display = ['.'; 240];

    solve(input, |simulation| {
        let pixel_positions = (simulation.register - 1)..=(simulation.register + 1);

        let horizontal_position = (simulation.cycle - 1) % 40;

        if pixel_positions.contains(&horizontal_position) {
            display[(simulation.cycle - 1) as usize] = '#';
        }
    });

    let formatted_display = display
        .chunks(40)
        .map(|row| row.iter().collect::<String>())
        .join("\n");

    formatted_display
}

pub fn main() {
    let input = include_str!("../input/day10.txt");

    execution_time(|| dbg!(solve_part1(input)));

    execution_time(|| println!("Part 2: \n{}", solve_part2(input)));
}
