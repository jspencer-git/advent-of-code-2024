use utils::*;
use std::{collections::HashSet, fmt, str::FromStr};

#[derive(PartialEq, Eq)]
enum GridElem {
    Digit(u32)
}

impl From<char> for GridElem {
    fn from(item: char) -> Self {
        Self::Digit(item.to_digit(10).unwrap())
    }
}

impl fmt::Display for GridElem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Self::Digit(d) => d.to_string()
        };
        write!(f, "{}", ch)
    }
}

fn solve(input: &Grid<GridElem>, idx: usize, current: u32, peaks: &mut HashSet<usize>) -> i64 {
    match input.get(idx) {
        GridElem::Digit(val) => { if current != *val { return 0; } }
    }

    let pos = input.get_xy_from_idx(idx).unwrap();
    println!("Found {} at {:?}", current, pos);

    if current == 9 {
        if peaks.contains(&idx) {
            return 0;
        }

        peaks.insert(idx);
        return 1;
    }

    const DIRS: [(i64, i64); 4] = [
        (-1, 0),    // Left
        (1, 0),    // Right
        (0, 1),    // Up
        (0, -1)    // Down
    ];

    let mut sum: i64 = 0;
    for dir in DIRS {
        match input.get_idx_from_xy((pos.0 + dir.0, pos.1 + dir.1)) {
            Some(idx) => { sum += solve(input, idx,  current + 1, peaks); },
            None => { continue; }
        }
    }

    sum
}

fn part_one(path: &str) {
    let input = Grid::<GridElem>::from_str(&get_input(path)).unwrap();
    println!("{}", input);

    let mut score: i64 = 0;
    for idx in 0..input.data.len() {
        let mut peaks = HashSet::new();
        let current = solve(&input, idx, 0, &mut peaks);
        println!("Score for idx {} is {}", idx, current);
        score += current;
    }

    println!("{}", score);
}

fn solve2(input: &Grid<GridElem>, idx: usize, current: u32) -> i64 {
    match input.get(idx) {
        GridElem::Digit(val) => { if current != *val { return 0; } }
    }

    let pos = input.get_xy_from_idx(idx).unwrap();
    println!("Found {} at {:?}", current, pos);

    if current == 9 {
        return 1;
    }

    const DIRS: [(i64, i64); 4] = [
        (-1, 0),    // Left
        (1, 0),    // Right
        (0, 1),    // Up
        (0, -1)    // Down
    ];

    let mut sum: i64 = 0;
    for dir in DIRS {
        match input.get_idx_from_xy((pos.0 + dir.0, pos.1 + dir.1)) {
            Some(idx) => { sum += solve2(input, idx,  current + 1); },
            None => { continue; }
        }
    }

    sum
}

fn part_two(path: &str) {
    let input = Grid::<GridElem>::from_str(&get_input(path)).unwrap();
    println!("{}", input);

    let mut score: i64 = 0;
    for idx in 0..input.data.len() {
        let current = solve2(&input, idx, 0);
        println!("Score for idx {} is {}", idx, current);
        score += current;
    }

    println!("{}", score);
}

fn main() {
    println!("Part one:");
    // 782
    part_one("day-10/input.txt");

    println!("Part two:");
    // 1694
    part_two("day-10/input.txt");
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
