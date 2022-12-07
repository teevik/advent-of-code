#![feature(is_some_and)]
#![feature(hash_raw_entry)]

use advent_of_code::execution_time;
use anyhow::{Context, Result};
use slotmap::{new_key_type, SecondaryMap, SlotMap};
use std::collections::HashMap;

new_key_type! { struct DirectoryKey; }

#[derive(Debug)]
struct Directories<'a> {
    directories: SlotMap<DirectoryKey, Directory<'a>>,
    root_directory: DirectoryKey,
    directory_sizes: SecondaryMap<DirectoryKey, u64>,
}

impl<'a> Directories<'a> {
    pub fn new() -> Self {
        let mut directories = SlotMap::with_key();
        let root_directory = directories.insert(Directory::default());

        Self {
            directories,
            root_directory,
            directory_sizes: Default::default(),
        }
    }

    pub fn insert(&mut self, path: &[&'a str], files_size: u64) {
        dbg!(path);
        let mut parent_directory_key = self.root_directory;

        let directory_size = self
            .directory_sizes
            .entry(parent_directory_key)
            .unwrap()
            .or_default();

        *directory_size += files_size;

        for &path_part in path {
            let directory_key = {
                let directory = self.directories.get_mut(parent_directory_key).unwrap();

                directory.sub_directories.get(path_part).copied()
            };

            let directory_key =
                directory_key.unwrap_or_else(|| self.directories.insert(Default::default()));

            self.directories
                .get_mut(parent_directory_key)
                .unwrap()
                .sub_directories
                .insert(path_part, directory_key);

            let directory_size = self
                .directory_sizes
                .entry(directory_key)
                .unwrap()
                .or_default();

            *directory_size += files_size;
        }
    }

    pub fn update_price() {}
}

#[derive(Default, Debug)]
struct Directory<'a> {
    pub sub_directories: HashMap<&'a str, DirectoryKey>,
}

pub fn solve_part1(input: &str) -> Result<u64> {
    let mut lines = input.lines().peekable();

    let mut path = Vec::new();
    let mut directories = Directories::new();

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
                    path.push(move_to);
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

            directories.insert(&path, files_size);
        }
    }

    dbg!(&directories);

    let result = directories
        .directory_sizes
        .values()
        .filter(|&&directory_size| directory_size <= 100000)
        .sum();

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
