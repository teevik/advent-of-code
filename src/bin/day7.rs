#![feature(is_some_and)]
#![feature(hash_raw_entry)]

use advent_of_code::execution_time;
use anyhow::{Context, Result};
use std::collections::HashMap;

#[derive(Default, Debug)]
struct Directory {
    pub sub_directories: HashMap<String, Directory>,
    pub files_size: u64,
}

impl Directory {
    pub fn entry(&mut self, path: &[String]) -> &mut Directory {
        let mut directory = self;

        for path_part in path {
            directory = directory
                .sub_directories
                .raw_entry_mut()
                .from_key(path_part)
                .or_insert_with(|| (path_part.clone(), Default::default()))
                .1;
        }

        directory
    }

    pub fn total_size(&self) -> u64 {
        self.files_size
            + self
                .sub_directories
                .values()
                .map(|directory| directory.total_size())
                .sum::<u64>()
    }
}

pub fn solve_part1(input: &str) -> Result<u64> {
    let mut lines = input.lines().peekable();

    let mut path = Vec::new();
    let mut root_directory = Directory::default();

    while lines.peek().is_some() {
        let line = lines.next().unwrap();

        if let Some(cd) = line.strip_prefix("$ cd ") {
            match cd {
                "/" => {
                    path.clear();
                }
                ".." => {
                    path.pop();
                }
                move_to => {
                    path.push(move_to.to_string());
                }
            }
        } else if line.strip_prefix("$ ls").is_some() {
            let mut files_size = 0;

            while lines.peek().is_some_and(|line| !line.starts_with('$')) {
                let line = lines.next().unwrap();

                let first_word = line.split(" ").next().context("Parsing error")?;

                if first_word == "dir" {
                    continue;
                };

                let size = first_word.parse::<u64>()?;

                files_size += size;
            }

            root_directory.entry(&path).files_size = files_size;
        }
    }

    fn find_size_recursively(directory: &Directory) -> u64 {
        let sub_directory_values = directory
            .sub_directories
            .values()
            .map(|directory| find_size_recursively(directory))
            .sum::<u64>();

        let self_size = directory.total_size();
        let self_size = if self_size <= 100000 { self_size } else { 0 };

        self_size + sub_directory_values
    }

    dbg!(&root_directory);
    let result = find_size_recursively(&root_directory);

    Ok(result)
}

pub fn main() -> Result<()> {
    let input = include_str!("../input/day7.txt");

    let part_1 = execution_time(|| solve_part1(input));
    dbg!(part_1);

    // let part_2 = execution_time(|| solve_part2(input))?;
    // dbg!(part_2);

    Ok(())
}
