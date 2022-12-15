use advent_of_code::execution_time;
use advent_of_code::iterator_helpers::IteratorHelpers;
use anyhow::Result;
use itertools::Either::{Left, Right};
use ndarray::{Array, Axis, Ix2};
use take_until::TakeUntilExt;

fn parse_height_map(input: &str) -> Result<Array<i32, Ix2>> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let input_digits = input
        .lines()
        .flat_map(|line| line.chars().map(|char| char.to_digit(10).unwrap() as i32));

    let height_map = Array::from_iter(input_digits).into_shape((width, height))?;

    Ok(height_map)
}

fn solve_part1(input: &str) -> Result<i32> {
    let height_map = parse_height_map(input)?;
    let mut visited_items = Array::<bool, _>::default(height_map.raw_dim());

    let get_score_in_direction =
        |axis: Axis, reverse: bool, visited: &mut Array<bool, Ix2>| -> i32 {
            let mut count = 0;

            for (axis_index, view) in height_map.axis_iter(axis).enumerate() {
                let mut max_height = -1;

                for (index, &target_height) in view.iter().enumerate().rev_if(reverse) {
                    if target_height > max_height {
                        max_height = target_height;

                        let mut visited_view = visited.index_axis_mut(axis, axis_index);
                        let has_been_visited = visited_view.get_mut(index).unwrap();

                        if !*has_been_visited {
                            *has_been_visited = true;
                            count += 1;
                        }
                    }
                }
            }

            count
        };

    let score = get_score_in_direction(Axis(0), false, &mut visited_items)
        + get_score_in_direction(Axis(1), false, &mut visited_items)
        + get_score_in_direction(Axis(0), true, &mut visited_items)
        + get_score_in_direction(Axis(1), true, &mut visited_items);

    Ok(score)
}
fn solve_part2(input: &str) -> Result<usize> {
    let height_map = parse_height_map(input)?;

    let scenic_score_side =
        |target_tree: i32, axis: Axis, reversed: bool, axis_index: usize, index: usize| {
            let axis_view = height_map.index_axis(axis, axis_index);

            let side_iterator = if reversed {
                Left(axis_view.iter().take(index).rev())
            } else {
                Right(axis_view.iter().skip(index + 1))
            };

            let score = side_iterator
                .take_until(|&&other_tree| target_tree <= other_tree)
                .count();

            score
        };

    let scenic_score = |x: usize, y: usize, target_tree: i32| {
        let score = scenic_score_side(target_tree, Axis(0), false, x, y)
            * scenic_score_side(target_tree, Axis(0), true, x, y)
            * scenic_score_side(target_tree, Axis(1), false, y, x)
            * scenic_score_side(target_tree, Axis(1), true, y, x);

        score
    };

    let biggest_scenic_score = height_map
        .indexed_iter()
        .map(|((x, y), &target_tree)| scenic_score(x, y, target_tree))
        .max()
        .unwrap();

    Ok(biggest_scenic_score)
}

pub fn main() -> Result<()> {
    let input = include_str!("../input/day8.txt");

    let part_1 = execution_time(|| solve_part1(input))?;
    dbg!(part_1);

    let part_2 = execution_time(|| solve_part2(input))?;
    dbg!(part_2);

    Ok(())
}
