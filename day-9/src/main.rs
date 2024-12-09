use std::collections::BTreeMap;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::str::FromStr;
use std::{collections::VecDeque, fs::File, io::Read};
use std::{fmt, result};
use utils::*;

#[derive(Debug, PartialEq)]
enum BlockType {
    File(i64),
    Space,
}
#[derive(Debug, PartialEq)]
struct Block {
    block_type: BlockType,
    start: usize,
    size: usize,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.block_type {
                BlockType::Space => ".".repeat(self.size),
                BlockType::File(n) => n.to_string().repeat(self.size),
            }
        )
    }
}

struct Disk {
    map: BTreeMap<usize, Block>,
}

impl Disk {
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in self.map.iter() {
            write!(f, "{}", x.1)?
        }

        Ok(())
    }
}

fn part_one(path: &str) {
    let mut input = get_input(path);
    input.retain(|x| x != '\n');

    let input_numbers: Vec<usize> = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();

    let mut disk = Disk::new();

    let mut id = 0;
    let mut is_block = true;
    let mut offset: usize = 0;
    for n in input_numbers.iter() {
        if is_block {
            if *n > 0 {
                disk.map.insert(
                    offset,
                    Block {
                        block_type: BlockType::File(id),
                        start: offset,
                        size: *n,
                    },
                );
            }
            id += 1;
        } else {
            if *n > 0 {
                disk.map.insert(
                    offset,
                    Block {
                        block_type: BlockType::Space,
                        start: offset,
                        size: *n,
                    },
                );
            }
        }

        offset += *n;
        is_block = !is_block;
    }

    println!("DISK: {}", disk);
}

fn part_two(path: &str) {}

fn main() {
    println!("Part one:");
    part_one("day-9/input.txt");

    println!("Part two:");
    part_two("day-9/input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        println!(
            "CWD: {}",
            std::env::current_dir().unwrap().to_str().unwrap()
        );
        part_one("example-input.txt");
    }

    #[test]
    fn test_part_two() {
        println!(
            "CWD: {}",
            std::env::current_dir().unwrap().to_str().unwrap()
        );
        part_two("example-input.txt");
    }
}
