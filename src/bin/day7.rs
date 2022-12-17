use advent_of_code::execution_time;
use anyhow::Result;
use hashbrown::HashMap;
use itertools::Itertools;
use slotmap::{new_key_type, SecondaryMap, SlotMap};

new_key_type! { struct DirectoryKey; }

#[derive(Debug)]
struct Directories {
    all_directories: SlotMap<DirectoryKey, Directory>,
    root_directory: DirectoryKey,
    directory_sizes: SecondaryMap<DirectoryKey, u64>,
}

impl Directories {
    pub fn new() -> Self {
        let mut all_directories = SlotMap::default();

        let root_directory = all_directories.insert(Directory::default());

        Self {
            all_directories,
            root_directory,
            directory_sizes: Default::default(),
        }
    }

    fn get_directory(&self, directory_key: DirectoryKey) -> &Directory {
        self.all_directories.get(directory_key).unwrap()
    }

    fn get_directory_mut(&mut self, directory_key: DirectoryKey) -> &mut Directory {
        self.all_directories.get_mut(directory_key).unwrap()
    }

    fn get_or_insert_subdirectory(
        &mut self,
        directory_key: DirectoryKey,
        path: &'static str,
    ) -> DirectoryKey {
        let subdirectory_key = self
            .get_directory(directory_key)
            .children
            .get(path)
            .copied();

        if let Some(subdirectory_key) = subdirectory_key {
            subdirectory_key
        } else {
            let new_subdirectory_key = self.all_directories.insert(Default::default());

            self.get_directory_mut(directory_key)
                .children
                .insert(path, new_subdirectory_key);

            new_subdirectory_key
        }
    }

    pub fn add_directory_size(&mut self, path: &[&'static str], size: u64) {
        let mut current_directory = self.root_directory;

        let mut path_iterator = path.iter();

        while true {
            let directory_size = self
                .directory_sizes
                .entry(current_directory)
                .unwrap()
                .or_default();
            *directory_size += size;

            let Some(path_part) = path_iterator.next() else { break };

            let subdirectory_key = self.get_or_insert_subdirectory(current_directory, path_part);
            current_directory = subdirectory_key;
        }
    }
}

#[derive(Debug, Default)]
struct Directory {
    children: HashMap<&'static str, DirectoryKey>,
}

fn parse_directories(input: &'static str) -> Directories {
    let mut directories = Directories::new();

    let mut lines = input.lines().peekable();
    let mut path = Vec::new();

    while let Some(next_line) = lines.next() {
        // Skip first `$`
        let mut line_parts = next_line.split(' ').skip(1);

        let command = line_parts.next().unwrap();

        match command {
            "cd" => {
                let cd_path = line_parts.next().unwrap();

                match cd_path {
                    ".." => {
                        path.pop();
                    }

                    "/" => {
                        path.clear();
                    }

                    cd_path => {
                        path.push(cd_path);
                    }
                }
            }

            "ls" => {
                for ls_line in lines.peeking_take_while(|&line| !line.starts_with('$')) {
                    let (size_or_dir, _) = ls_line.split_once(' ').unwrap();

                    if !size_or_dir.starts_with("dir") {
                        let size = size_or_dir.parse::<u64>().unwrap();

                        directories.add_directory_size(&path, size);
                    }
                }
            }

            _ => unreachable!(),
        }
    }

    directories
}

pub fn solve_part1(input: &'static str) -> u64 {
    let directories = parse_directories(input);

    let total_size = directories
        .directory_sizes
        .values()
        .filter(|&&size| size <= 100000)
        .sum();

    total_size
}

pub fn solve_part2(input: &'static str) -> u64 {
    let directories = parse_directories(input);

    const DISK_SIZE: u64 = 70_000_000;
    const NEEDED_FREE_SPACE: u64 = 30_000_000;

    let root_size = *directories
        .directory_sizes
        .get(directories.root_directory)
        .unwrap();
    let available = DISK_SIZE - root_size;

    let smallest_deletable_size = *directories
        .directory_sizes
        .values()
        .filter(|&&size| available + size >= NEEDED_FREE_SPACE)
        .min()
        .unwrap();

    smallest_deletable_size
}

pub fn main() -> Result<()> {
    let input = include_str!("../input/day7.txt");

    let part_1 = execution_time(|| solve_part1(input));
    dbg!(part_1);

    let part_2 = execution_time(|| solve_part2(input));
    dbg!(part_2);

    Ok(())
}
