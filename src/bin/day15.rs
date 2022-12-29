use advent_of_code::execution_time;
use anyhow::Context;
use itertools::{Either, Itertools};
use rayon::prelude::*;
use vek::Vec2;

fn manhattan_distance(a: Vec2<i32>, b: Vec2<i32>) -> u32 {
    i32::abs_diff(a.x, b.x) + i32::abs_diff(a.y, b.y)
}

struct Measurement {
    sensor_position: Vec2<i32>,
    beacon_position: Vec2<i32>,
    distance: u32,
}

impl Measurement {
    pub fn new(sensor_position: Vec2<i32>, beacon_position: Vec2<i32>) -> Self {
        Self {
            sensor_position,
            beacon_position,
            distance: manhattan_distance(sensor_position, beacon_position),
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Measurement> + '_ {
    input.lines().map(|line| {
        let mut parts = line.split(['=', ',', ':']);

        fn parse(number: Option<&str>) -> anyhow::Result<i32> {
            let number = number.context("Parsing error")?;
            let number = number.parse()?;

            Ok(number)
        }

        let sensor_x = parse(parts.nth(1)).unwrap();
        let sensor_y = parse(parts.nth(1)).unwrap();
        let beacon_x = parse(parts.nth(1)).unwrap();
        let beacon_y = parse(parts.nth(1)).unwrap();

        Measurement::new(Vec2::new(sensor_x, sensor_y), Vec2::new(beacon_x, beacon_y))
    })
}

fn solve_part1(input: &str) -> usize {
    let measurements = parse_input(input).collect_vec();

    let min_x = measurements
        .iter()
        .map(|measurement| measurement.sensor_position.x - measurement.distance as i32)
        .min()
        .unwrap();

    let max_x = measurements
        .iter()
        .map(|measurement| measurement.sensor_position.x + measurement.distance as i32)
        .max()
        .unwrap();

    let y = 2_000_000;

    let all_x = min_x..=max_x;

    let result = all_x
        .into_par_iter()
        .filter(|&x| {
            let position = Vec2::new(x, y);
            measurements.iter().any(|measurement| {
                let distance = manhattan_distance(position, measurement.sensor_position);

                let is_in_sensor_distance = distance <= measurement.distance;
                let existing_beacon = measurement.beacon_position == position;

                is_in_sensor_distance && !existing_beacon
            })
        })
        .count();

    result
}

fn solve_part2(input: &str) -> Option<u64> {
    let measurements = parse_input(input).collect_vec();

    let max = 4_000_000;

    let found_result = measurements.iter().find_map(|measurement| {
        let Measurement {
            sensor_position,
            distance,
            ..
        } = *measurement;

        let mut dx = 0;

        let clamp = |a: i32| a.clamp(0, max);

        let vertical_range = clamp(sensor_position.y - distance as i32 + 1)
            ..=clamp(sensor_position.y + distance as i32 + 1);

        for y in vertical_range {
            let targets = if dx == 0 {
                Either::Left([Vec2::new(sensor_position.x, y)])
            } else {
                Either::Right([
                    Vec2::new(sensor_position.x + dx, y),
                    Vec2::new(sensor_position.x - dx, y),
                ])
            };

            let target_position = targets
                .into_iter()
                .filter(|position| position.x >= 0 && position.x <= max)
                .find(|&position| {
                    measurements.iter().all(|measurement| {
                        let distance = manhattan_distance(position, measurement.sensor_position);

                        let is_in_sensor_distance = distance <= measurement.distance;

                        !is_in_sensor_distance
                    })
                });

            if let Some(target_position) = target_position {
                return Some(target_position.x as u64 * max as u64 + target_position.y as u64);
            }

            if y <= sensor_position.y {
                dx += 1;
            } else {
                dx -= 1
            }
        }

        None
    });

    return found_result;
}

pub fn main() {
    let input = include_str!("../input/day15.txt");

    execution_time(|| dbg!(solve_part1(input)));

    execution_time(|| dbg!(solve_part2(input)));
}
