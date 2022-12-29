use advent_of_code::execution_time;
use extent::Extent;
use hashbrown::HashSet;
use itertools::{Either, Itertools};
use vek::Vec2;

struct ParseResult {
    tiles: HashSet<Vec2<i32>>,
    max_y: i32,
}

fn parse_input(input: &str) -> ParseResult {
    let stone_paths = input.split("\n").map(|path| {
        path.split(" -> ").map(|point| {
            let (x, y) = point.split_once(',').unwrap();
            let (x, y) = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());

            Vec2::<i32>::new(x, y)
        })
    });

    let max_y = stone_paths
        .clone()
        .flatten()
        .map(|point| point.y)
        .max()
        .unwrap();

    let stone_points = stone_paths.flat_map(|stone_path| {
        stone_path.tuple_windows::<(_, _)>().flat_map(|(a, b)| {
            let difference = a - b;

            let positions_between = match difference {
                Vec2 { x: 0, y: _y } => {
                    Either::Left(Extent::new(a.y, b.y).iter().map(move |y| a.with_y(y)))
                }
                Vec2 { x: _x, y: 0 } => {
                    Either::Right(Extent::new(a.x, b.x).iter().map(move |x| a.with_x(x)))
                }
                _ => unreachable!(),
            };

            positions_between
        })
    });

    let tiles = HashSet::from_iter(stone_points);

    ParseResult { tiles, max_y }
}

fn solve_part1(input: &str) -> i32 {
    let ParseResult { max_y, mut tiles } = parse_input(input);

    let sand_spawn = Vec2::new(500, 0);

    let mut sand_in_rest = 0;

    loop {
        let mut sand_position = sand_spawn;

        loop {
            let mut try_move = |offset: Vec2<i32>| {
                if !tiles.contains(&(sand_position + offset)) {
                    sand_position += offset;
                    true
                } else {
                    false
                }
            };

            const DOWN: Vec2<i32> = Vec2::new(0, 1);
            const DOWN_LEFT: Vec2<i32> = Vec2::new(-1, 1);
            const DOWN_RIGHT: Vec2<i32> = Vec2::new(1, 1);

            let did_move = try_move(DOWN) || try_move(DOWN_LEFT) || try_move(DOWN_RIGHT);

            if !did_move {
                tiles.insert(sand_position);
                sand_in_rest += 1;

                break;
            } else if sand_position.y > max_y {
                return sand_in_rest;
            }
        }
    }
}

fn solve_part2(input: &str) -> i32 {
    let ParseResult { max_y, mut tiles } = parse_input(input);
    let floor = max_y + 1;

    let sand_spawn = Vec2::new(500, 0);

    let mut sand_in_rest = 0;

    loop {
        let mut sand_position = sand_spawn;

        loop {
            let mut try_move = |offset: Vec2<i32>| {
                if !tiles.contains(&(sand_position + offset)) {
                    sand_position += offset;
                    true
                } else {
                    false
                }
            };

            const DOWN: Vec2<i32> = Vec2::new(0, 1);
            const DOWN_LEFT: Vec2<i32> = Vec2::new(-1, 1);
            const DOWN_RIGHT: Vec2<i32> = Vec2::new(1, 1);

            let did_move = try_move(DOWN) || try_move(DOWN_LEFT) || try_move(DOWN_RIGHT);

            if !did_move || sand_position.y == floor {
                tiles.insert(sand_position);
                sand_in_rest += 1;

                if sand_position == sand_spawn {
                    return sand_in_rest;
                }

                break;
            }
        }
    }
}

pub fn main() {
    let input = include_str!("../input/day14.txt");

    execution_time(|| dbg!(solve_part1(input)));

    execution_time(|| dbg!(solve_part2(input)));
}
