use advent_of_code::execution_time;
use ndarray::Array2;
use pathfinding::prelude::dijkstra;
use rayon::prelude::*;
use vek::Vec2;

type Elevations = Array2<u8>;

struct ParseResult {
    elevations: Elevations,
    trail_starts: Vec<Vec2<usize>>,
    start: Vec2<usize>,
    end: Vec2<usize>,
}

fn parse_input(input: &str) -> ParseResult {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut start = Vec2::default();
    let mut end = Vec2::default();

    let mut elevations = Array2::default((width, height));
    let mut trail_starts = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, mut elevation) in line.bytes().enumerate() {
            match elevation {
                b'a' => {
                    trail_starts.push(Vec2::new(x, y));
                }
                b'S' => {
                    elevation = b'a';
                    start = Vec2::new(x, y);
                }
                b'E' => {
                    elevation = b'z';
                    end = Vec2::new(x, y);
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

fn is_in_bounds(bounds: Vec2<usize>, position: Vec2<i32>) -> Option<Vec2<usize>> {
    (position.x >= 0
        && position.x < bounds.x as i32
        && position.y >= 0
        && position.y < bounds.y as i32)
        .then(|| (position.as_()))
}

fn find_shortest_path(
    start: Vec2<usize>,
    end: Vec2<usize>,
    elevations: &Elevations,
) -> Option<u32> {
    let bounds = Vec2::from(elevations.dim());

    let (_, steps) = dijkstra(
        &start,
        |&position| {
            let elevation = elevations[position.into_tuple()];

            const NEIGHBOR_OFFSETS: [Vec2<i32>; 4] = [
                Vec2::new(0, -1),
                Vec2::new(1, 0),
                Vec2::new(0, 1),
                Vec2::new(-1, 0),
            ];

            NEIGHBOR_OFFSETS.iter().flat_map(move |&neighbor_offset| {
                let neighbor_position = position.as_::<i32>() + neighbor_offset;

                is_in_bounds(bounds, neighbor_position).and_then(|neighbor_position| {
                    let neighbor_elevation = elevations[neighbor_position.into_tuple()];

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
