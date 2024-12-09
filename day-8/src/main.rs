use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::{collections::VecDeque, fs::File, io::Read};
use std::{fmt, result};
use utils::*;

#[derive(Clone, Debug, PartialEq)]
enum GridElem {
    Empty,
    Object(char),
    Antinode,
}

impl From<char> for GridElem {
    fn from(item: char) -> Self {
        match item {
            '.' => Self::Empty,
            '#' => unreachable!(),
            ch => Self::Object(ch),
        }
    }
}

impl fmt::Display for GridElem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Self::Empty => '.',
            Self::Object(ch) => *ch,
            Self::Antinode => '#',
            _ => unreachable!(),
        };
        write!(f, "{}", ch)
    }
}

fn part_one(path: &str) {
    let input = get_input(path);
    let g: Grid<GridElem> = Grid::from_str(&input).unwrap();
    let mut results_g = g.clone();
    let mut antennas: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();
    println!("{}", g);

    for (i, elem) in g.data.iter().enumerate() {
        match elem {
            GridElem::Object(ch) => {
                if !antennas.contains_key(ch) {
                    antennas.insert(*ch, Vec::new());
                }
                antennas
                    .get_mut(ch)
                    .unwrap()
                    .push(g.get_xy_from_idx(i).unwrap());
            }
            _ => {}
        }
    }
    println!("antennas {:?}", antennas);

    for (antenna_type, locations) in antennas.iter() {
        for loc in locations.iter() {
            for other in locations.iter() {
                if other == loc {
                    continue;
                }

                let diff = (other.0 - loc.0, other.1 - loc.1);
                let antinode_loc = (loc.0 - diff.0, loc.1 - diff.1);
                if let Some(idx) = g.get_idx_from_xy(antinode_loc) {
                    antinodes.insert(antinode_loc);
                    if let GridElem::Object(_) = results_g.get(idx) {
                    } else {
                        results_g.set(idx, GridElem::Antinode);
                    }
                };
            }
        }
    }

    println!("{}", results_g);
    println!("{}", antinodes.len());
}

fn part_two(path: &str) {}

fn main() {
    println!("Part one:");
    // not 356 - too high
    part_one("day-8/input.txt");

    println!("Part two:");
    part_two("day-8/input.txt");
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
