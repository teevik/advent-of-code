use advent_of_code::execution_time;
use anyhow::Result;

struct Node {
    value: u64,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Node {
    fn is_directory(&self) -> bool {
        self.children.len() > 0
    }
}

struct FileTree {
    files: Vec<Node>,
}

impl FileTree {
    fn insert(&mut self, value: u64, parent: Option<usize>) -> Option<usize> {
        let index = self.files.len();
        if let Some(parent) = parent {
            self.files[parent].children.push(index);
            self.update_size(Some(parent), value);
        }
        self.files.push(Node {
            value,
            parent,
            children: vec![],
        });
        Some(index)
    }

    fn node(&self, index: Option<usize>) -> &Node {
        match index {
            Some(index) => &self.files[index],
            None => panic!("Trying to access a non-existing node"),
        }
    }

    fn update_size(&mut self, index: Option<usize>, size: u64) {
        let mut current = index;
        while let Some(index) = current {
            let node = &mut self.files[index];
            node.value += size;
            current = node.parent;
        }
    }
}

const MAX_DIR_SIZE: u64 = 100000;
const TOTAL_DISK_SIZE: u64 = 70000000;
const UPDATE_SIZE: u64 = 30000000;

fn parse_file_tree(input: &str) -> FileTree {
    let mut file_tree = FileTree { files: vec![] };
    let mut current: Option<usize> = None;

    input.lines().for_each(|line| {
        let splited: Vec<&str> = line.split(" ").collect();
        match splited[0] {
            "$" => match splited[1] {
                "cd" => match splited[2] {
                    ".." => {
                        current = file_tree.node(current).parent;
                    }
                    _ => {
                        current = file_tree.insert(0, current);
                    }
                },
                _ => {}
            },
            "dir" => {}
            _ => {
                file_tree.insert(splited[0].parse::<u64>().unwrap(), current);
            }
        }
    });

    file_tree
}

pub fn solve_part1(input: &str) -> Result<u64> {
    let file_tree = parse_file_tree(input);

    let directories = file_tree
        .files
        .into_iter()
        .filter(|node| node.is_directory());

    let result = directories
        .filter(|node| node.value <= MAX_DIR_SIZE)
        .map(|node| node.value)
        .sum();

    Ok(result)
}

pub fn solve_part2(input: &str) -> Result<u64> {
    let file_tree = parse_file_tree(input);

    let directories = file_tree.files.iter().filter(|node| node.is_directory());

    let to_delete_size = UPDATE_SIZE - (TOTAL_DISK_SIZE - file_tree.files[0].value);
    let result = directories
        .filter(|node| node.value >= to_delete_size)
        .map(|node| node.value)
        .min()
        .unwrap();

    Ok(result)
}

pub fn main() -> Result<()> {
    let input = include_str!("../input/day7.txt");

    let part_1 = execution_time(|| solve_part1(input))?;
    dbg!(part_1);

    let part_2 = execution_time(|| solve_part2(input))?;
    dbg!(part_2);

    Ok(())
}
