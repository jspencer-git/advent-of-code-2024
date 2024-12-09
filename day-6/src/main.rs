use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::Empty;
use std::str::FromStr;
use std::thread::current;
use std::{fs::File, io::Read};
use utils::*;

#[derive(Clone, Debug, PartialEq)]
enum GridElem {
    Empty,
    Obstacle,
    Visited,
    Guard,
}

impl From<char> for GridElem {
    fn from(item: char) -> Self {
        match item {
            '#' => Self::Obstacle,
            '^' => Self::Guard,
            '.' => Self::Empty,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for GridElem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Self::Obstacle => '#',
            Self::Visited => 'X',
            Self::Guard => '^',
            Self::Empty => '.',
            _ => unreachable!(),
        };
        write!(f, "{}", ch)
    }
}

fn part_one(path: &str) {
    let input = get_input(path);
    let mut g: Grid<GridElem> = Grid::from_str(&input).unwrap();
    let mut g_result = g.clone();

    let guard_start = g
        .data
        .iter()
        .enumerate()
        .find_map(|(i, x)| if *x == GridElem::Guard { Some(i) } else { None })
        .unwrap();

    let mut visited = HashSet::new();

    // Guard starts facing up.
    let mut current_dir = (0, -1);
    let mut current_xy = g.get_xy_from_idx(guard_start).unwrap();

    loop {
        let next_xy = (current_xy.0 + current_dir.0, current_xy.1 + current_dir.1);
        let next_idx = g.get_idx_from_xy(next_xy);
        if let Some(idx) = next_idx {
            match g.get(idx) {
                GridElem::Obstacle => {
                    current_dir = match current_dir {
                        (0, -1) => (1, 0),
                        (1, 0) => (0, 1),
                        (0, 1) => (-1, 0),
                        (-1, 0) => (0, -1),
                        _ => unreachable!(),
                    }
                }
                _ => {}
            }
        }

        if let Some(idx) = g.get_idx_from_xy(current_xy) {
            visited.insert(idx);
            g_result.set(idx, GridElem::Visited);
        }

        let next_xy = (current_xy.0 + current_dir.0, current_xy.1 + current_dir.1);
        let next_idx = g.get_idx_from_xy(next_xy);
        if next_idx.is_none() {
            break;
        }

        current_xy = next_xy;
    }

    println!("{}", g_result);
    println!("{}", visited.len())
}

#[derive(Clone, Debug, PartialEq)]
enum GridElem2 {
    Empty,
    Obstacle,
    AddedObstacle,
    Visited((i64, i64)),
    Guard,
}

impl From<char> for GridElem2 {
    fn from(item: char) -> Self {
        match item {
            '#' => Self::Obstacle,
            '^' => Self::Guard,
            '.' => Self::Empty,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for GridElem2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Self::Obstacle => '#',
            Self::AddedObstacle => 'O',
            Self::Visited(_) => 'X',
            Self::Guard => '^',
            Self::Empty => '.',
            _ => unreachable!(),
        };
        write!(f, "{}", ch)
    }
}

fn part_two(path: &str) {
    let input = get_input(path);
    let mut g: Grid<GridElem2> = Grid::from_str(&input).unwrap();

    let guard_start = g
        .data
        .iter()
        .enumerate()
        .find_map(|(i, x)| {
            if *x == GridElem2::Guard {
                Some(i)
            } else {
                None
            }
        })
        .unwrap();

    let mut obstacles = HashSet::new();

    // Guard starts facing up.
    let mut current_dir = (0, -1);
    let mut current_xy = g.get_xy_from_idx(guard_start).unwrap();

    loop {
        let next_xy = (current_xy.0 + current_dir.0, current_xy.1 + current_dir.1);
        let next_idx = g.get_idx_from_xy(next_xy);
        if let Some(idx) = next_idx {
            match g.get(idx) {
                GridElem2::Obstacle => {
                    current_dir = match current_dir {
                        (0, -1) => (1, 0),
                        (1, 0) => (0, 1),
                        (0, 1) => (-1, 0),
                        (-1, 0) => (0, -1),
                        _ => unreachable!(),
                    }
                }
                _ => {}
            }
        }

        let mut search_xy = current_xy;
        let next_dir: (i64, i64) = match current_dir {
            (0, -1) => (1, 0),
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            _ => unreachable!(),
        };
        let next_next_dir: (i64, i64) = match next_dir {
            (0, -1) => (1, 0),
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            _ => unreachable!(),
        };
        let mut last_dir = None;
        loop {
            if let Some(idx) = g.get_idx_from_xy(search_xy) {
                match g.get(idx) {
                    GridElem2::Visited(dir) => {
                        if *dir == next_dir {
                            if let Some(nidx) = next_idx {
                                obstacles.insert(nidx);
                                let mut g_copy = g.clone();
                                g_copy.set(nidx, GridElem2::AddedObstacle);
                                println!("{}", g_copy);
                                break;
                            }
                        }
                        last_dir = Some(*dir);
                    }
                    GridElem2::Obstacle => {
                        if let Some(ldir) = last_dir {
                            if ldir == next_next_dir {
                                if let Some(nidx) = next_idx {
                                    obstacles.insert(nidx);
                                    let mut g_copy = g.clone();
                                    g_copy.set(nidx, GridElem2::AddedObstacle);
                                    println!("{}", g_copy);
                                    break;
                                }
                            }
                        }
                    }
                    _ => {
                        last_dir = None;
                    }
                }
                search_xy = (search_xy.0 + next_dir.0, search_xy.1 + next_dir.1);
            } else {
                break;
            }
        }

        if let Some(idx) = g.get_idx_from_xy(current_xy) {
            g.set(idx, GridElem2::Visited(current_dir));
        }

        let next_xy = (current_xy.0 + current_dir.0, current_xy.1 + current_dir.1);
        let next_idx = g.get_idx_from_xy(next_xy);
        if next_idx.is_none() {
            break;
        }

        current_xy = next_xy;
    }

    println!("{}", obstacles.len())
}

fn main() {
    println!("Part one:");
    part_one("day-6/input.txt");

    println!("Part two:");
    // 332 too low
    // 1402 too low
    part_two("day-6/input.txt");
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
