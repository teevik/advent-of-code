use crate::neighborhood::Neighborhood;
use advent_of_code::execution_time;
use glam::IVec2;
use itertools::{iproduct, Itertools};
use ndarray::Array2;
use petgraph::matrix_graph::{DiMatrix, MatrixGraph, UnMatrix};
use petgraph::prelude::*;

mod neighborhood {
    use glam::IVec2;
    use std::iter::Flatten;

    pub struct Neighborhood {
        pub up: Option<u8>,
        pub right: Option<u8>,
        pub down: Option<u8>,
        pub left: Option<u8>,
    }

    pub struct IntoIter {
        iterator: Flatten<std::array::IntoIter<Option<(IVec2, u8)>, 4>>,
    }

    impl Iterator for IntoIter {
        type Item = (IVec2, u8);

        fn next(&mut self) -> Option<Self::Item> {
            self.iterator.next()
        }
    }

    impl IntoIterator for Neighborhood {
        type Item = (IVec2, u8);
        type IntoIter = IntoIter;

        fn into_iter(self) -> Self::IntoIter {
            let array = [
                self.up.map(|up| (IVec2::new(0, -1), up)),
                self.right.map(|right| (IVec2::new(1, 0), right)),
                self.down.map(|down| (IVec2::new(0, 1), down)),
                self.left.map(|left| (IVec2::new(-1, 0), left)),
            ];

            Self::IntoIter {
                iterator: array.into_iter().flatten(),
            }
        }
    }
}

struct Elevations {
    elevations: Vec<u8>,
    width: usize,
    height: usize,
}

impl Elevations {
    pub fn new(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let elevations = input.bytes().collect_vec();

        Self {
            elevations,
            width,
            height,
        }
    }

    pub fn get_elevation(&self, position: IVec2) -> Option<u8> {
        let IVec2 { x, y } = position;

        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return None;
        };

        let index = x + (y * (self.width as i32 + 1));
        let mut elevation = self.elevations[index as usize];

        if elevation == b'S' {
            elevation = b'a';
        } else if elevation == b'E' {
            elevation = b'z';
        }

        Some(elevation)
    }

    pub fn neighborhood(&self, position: IVec2) -> Neighborhood {
        Neighborhood {
            up: self.get_elevation(position + IVec2::new(0, -1)),
            right: self.get_elevation(position + IVec2::new(1, 0)),
            down: self.get_elevation(position + IVec2::new(0, 1)),
            left: self.get_elevation(position + IVec2::new(-1, 0)),
        }
    }

    // pub fn difference(&self, a: IVec2, b: IVec2) -> Option<u8> {
    //     let a = self.get_elevation(a)?;
    //     let b = self.get_elevation(b)?;
    //
    //     Some(u8::abs_diff(a, b))
    // }
}

fn pog() {}

struct ParseResult {
    elevations: Array2<u8>,
    start: (usize, usize),
    end: (usize, usize),
}

fn parse_input(input: &str) -> ParseResult {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut elevations = Array2::default((width, height));

    for (y, line) in input.lines().enumerate() {
        for (x, mut elevation) in line.bytes().enumerate() {
            if elevation == b'S' {
                elevation = b'a';
                start = (x, y);
            } else if elevation == b'E' {
                elevation = b'z';
                end = (x, y);
            }

            elevations[(x, y)] = elevation;
        }
    }

    ParseResult {
        elevations,
        start,
        end,
    }
}

fn solve_part1(input: &str) -> () {
    // let width = input.lines().next().unwrap().len();
    // let height = input.lines().count();
    //
    // let elevations = input.bytes().collect_vec();

    let elevations = Elevations::new(input);

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    // let get_elevations = |x: i32, y: i32| -> Option<u8> {
    //     if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
    //         return None;
    //     };
    //
    //     let index = x + (y * (width as i32 + 1));
    //     let elevation = elevations[index as usize];
    //
    //     Some(elevation)
    // };
    // let mut graph: MatrixGraph<(), i32> = MatrixGraph::new();
    // graph.add_edge();

    // GraphMap::from_edges(&[(IVec2::new(0, 0), IVec2::new(0, 1), 1)]);
    // graph.clear();

    // MatrixGraph::from_edges()

    let coords = iproduct!(0..(elevations.width as i32), 0..(elevations.height as i32));

    let a = coords.flat_map(|(x, y)| {
        let position = IVec2::new(x, y);

        let elevation = elevations.get_elevation(position).unwrap();

        let neighborhood = elevations.neighborhood(position);

        let edges = neighborhood
            .into_iter()
            .filter(move |&(_neighbor_offset, neighbor_elevation)| {
                elevation + 1 >= neighbor_elevation
            })
            .map(move |(neighbor_offset, _)| {
                let neighbor_position = position + neighbor_offset;

                let from = position.x as usize + (width as usize * (position.y as usize));
                let to = neighbor_position.x as usize
                    + (width as usize * (neighbor_position.y as usize));

                (from, to)
            });

        edges
    });

    let graph: DiMatrix<(), (), Option<()>, usize> = MatrixGraph::from_edges(a);

    // graph.
    // graph.

    // dbg!(graph.into_iter());

    // let a = (0..(elevations.width as i32)).flat_map(|x| (0..(elevations.height as i32)).map(|y| (x, y))).flat_map(||);

    for x in 0..(elevations.width as i32) {
        for y in 0..(elevations.height as i32) {
            let position = IVec2::new(x, y);

            let elevation = elevations.get_elevation(IVec2::new(x, y)).unwrap();

            // graph.

            let neighborhood = elevations.neighborhood(position);
        }
    }

    // towards right and down
    // UnMatrix::from_edges()

    // graph.
}

pub fn main() {
    let input = include_str!("../input/day12.txt");

    execution_time(|| dbg!(solve_part1(input)));

    // execution_time(|| dbg!(solve_part2(input)));
}
