use std::collections::{HashMap, HashSet};
use std::io::Empty;
use std::str::FromStr;
use std::{fs::File, io::Read};

fn get_input(path: &str) -> (String, String) {
    let file = File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(file);

    let mut input_data = String::new();
    reader.read_to_string(&mut input_data).unwrap();

    let inputs: Vec<&str> = input_data.split("\n\n").collect();
    assert!(inputs.len() == 2);

    (String::from(inputs[0]), String::from(inputs[1]))
}

#[derive(Debug, PartialEq, Eq)]
enum GridObject {
    Empty,
    Obstacle,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGridErr;

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    data: Vec<GridObject>,
    size: (i64),
    guard_start: (i64, i64)
}

// impl FromStr for Grid {
//     type Err = ParseGridErr;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let width = s.lines().next().unwrap().len();
//         let height = s.lines().count();

//         let input = 

//         let mut data: Vec<GridObject> = Vec::new();
//         let mut guard_start = None;

//         let mut idx = 0;
//         for ch in s.chars() {
//             match ch {
//                 '.' => data.push(Empty); idx += 1,
//                 '#' => data.push(GridObject); idx += 1,
//                 '^' => guard_start = Some()
//             }
//         }
//     }
// }

fn part_one(path: &str) {}

fn part_two() {}

fn main() {
    println!("Part one:");
    part_one("day-6/input.txt");

    println!("Part two:");
    part_two();
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
}
