use advent_of_code::execution_time;
use glam::IVec2;
use ndarray::Array2;
use pathfinding::prelude::dijkstra;
use rayon::prelude::*;

type Elevations = Array2<u8>;
type Position = (usize, usize);

struct ParseResult {
    elevations: Elevations,
    trail_starts: Vec<Position>,
    start: Position,
    end: Position,
}

fn parse_input(input: &str) -> ParseResult {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut elevations = Array2::default((width, height));
    let mut trail_starts = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, mut elevation) in line.bytes().enumerate() {
            match elevation {
                b'a' => {
                    trail_starts.push((x, y));
                }
                b'S' => {
                    elevation = b'a';
                    start = (x, y);
                }
                b'E' => {
                    elevation = b'z';
                    end = (x, y);
                }
                _ => {}
            }

            elevations[(x, y)] = elevation;
        }
    }

    ParseResult {
        elevations,
        trail_starts,
        start,
        end,
    }
}

fn is_in_bounds(bounds: Position, position: IVec2) -> Option<Position> {
    (position.x >= 0
        && position.x < bounds.0 as i32
        && position.y >= 0
        && position.y < bounds.1 as i32)
        .then(|| (position.x as usize, position.y as usize))
}

fn find_shortest_path(start: Position, end: Position, elevations: &Elevations) -> Option<u32> {
    let bounds = elevations.dim();

    let (_, steps) = dijkstra(
        &start,
        |&(x, y)| {
            let elevations = elevations.clone();
            let position = IVec2::new(x as i32, y as i32);
            let elevation = elevations[(x, y)];

            [
                IVec2::new(0, -1),
                IVec2::new(1, 0),
                IVec2::new(0, 1),
                IVec2::new(-1, 0),
            ]
            .into_iter()
            .flat_map(move |neighbor_offset| {
                let neighbor_position = position + neighbor_offset;

                is_in_bounds(bounds, neighbor_position).and_then(|neighbor_position| {
                    let neighbor_elevation = (&elevations)[neighbor_position];

                    let can_traverse = elevation + 1 >= neighbor_elevation;

                    can_traverse.then(|| (neighbor_position, 1))
                })
            })
        },
        |&position| position == end,
    )?;

    Some(steps)
}

fn solve_part1(input: &str) -> u32 {
    let ParseResult {
        elevations,
        start,
        end,
        ..
    } = parse_input(input);

    let elevations = elevations;

    let steps = find_shortest_path(start, end, &elevations);

    steps.expect("No path found")
}

fn solve_part2(input: &str) -> u32 {
    let ParseResult {
        elevations,
        trail_starts,
        end,
        ..
    } = parse_input(input);

    let elevations = elevations;

    let steps = trail_starts
        .par_iter()
        .flat_map(|&start| find_shortest_path(start, end, &elevations))
        .min()
        .unwrap();

    steps
}

pub fn main() {
    let input = include_str!("../input/day12.txt");

    execution_time(|| dbg!(solve_part1(input)));

    execution_time(|| dbg!(solve_part2(input)));
}
