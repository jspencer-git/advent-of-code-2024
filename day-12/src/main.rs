use utils::*;
use std::{collections::HashSet, fmt, str::FromStr};

#[derive(PartialEq, Eq, Hash)]
enum GridElem {
    Region(char)
}

impl From<char> for GridElem {
    fn from(item: char) -> Self {
        Self::Region(item)
    }
}

impl fmt::Display for GridElem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Self::Region(d) => d
        };
        write!(f, "{}", ch)
    }
}

fn solve(grid: &Grid<GridElem>, plant: char, idx: usize) -> i64 {
    let elem = grid.get(idx);
    if *elem != GridElem::Region(plant) {
        return 0;
    }

    // we are matching.
    let (x, y) = grid.get_xy_from_idx(idx).unwrap();

    let mut down: i64 = 0;
    let down_xy = (x, y + 1);
    if let Some(down_idx) = grid.get_idx_from_xy(down_xy) {
        down = solve(grid, plant, down_idx);
    }

    let mut right: i64 = 0;
    let right_xy = (x + 1, y);
    if let Some(right_idx) = grid.get_idx_from_xy(right_xy) {
        right = solve(grid, plant, right_idx);
    }
    
    1
}

fn part_one(path: &str) {
    let input: Grid<GridElem> = Grid::from_str(&get_input(path)).unwrap();
    let mut regions = HashSet::new();
    input.data.iter().for_each(|x| { regions.insert(x); });
    println!("{}", input);
}

fn part_two(path: &str) {
    let input = get_input(path);
}

fn main() {
    println!("Part one:");
    part_one("day-11/input.txt");

    println!("Part two:");
    part_two("day-11/input.txt");
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
