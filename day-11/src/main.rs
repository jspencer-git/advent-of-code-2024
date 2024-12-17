use std::collections::LinkedList;
use std::{
    collections::{HashMap, HashSet},
    fmt,
    str::FromStr,
};
use utils::*;

fn part_one(path: &str) {
    let input = get_input(path);
    let mut input: Vec<i64> = input
        .split(' ')
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();

    println!("{:?}", input);

    for i in 1..=25 {
        let mut input_next: Vec<i64> = Vec::new();
        for elem in input.iter() {
            match *elem {
                0 => {
                    input_next.push(1);
                }
                _ => {
                    let digit_str = elem.to_string();
                    let digit_count = digit_str.chars().count();

                    match digit_count % 2 {
                        0 => {
                            let (left, right) = digit_str.split_at(digit_count / 2);
                            let (left, right) =
                                (left.parse::<i64>().unwrap(), right.parse::<i64>().unwrap());
                            input_next.push(left);
                            input_next.push(right);
                        }
                        _ => {
                            input_next.push(elem * 2024);
                        }
                    }
                }
            }
        }
        input = input_next;
        // println!("{}: {:?}", i, input);
        println!("{}: {}", i, input.len());
    }
}

fn part_two(path: &str) {
    let input = get_input(path);
    let mut input: Vec<i64> = input
        .split(' ')
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();

    println!("{:?}", input);

    let mut map = HashMap::new();

    // Return how many stones an element at a given level will split into.
    fn solve(level_max: i64, map: &mut HashMap<(i64, i64), u64>, level: i64, elem: i64) -> u64 {
        if level == level_max {
            return 1;
        }

        if let Some(v) = map.get(&(level, elem)) {
            return *v;
        }

        let res = match elem {
            0 => {
               solve(level_max, map, level + 1, 1)
            }
            _ => {
                let digit_str = elem.to_string();
                let digit_count = digit_str.chars().count();

                match digit_count % 2 {
                    0 => {
                        let (left, right) = digit_str.split_at(digit_count / 2);
                        let (left, right) =
                            (left.parse::<i64>().unwrap(), right.parse::<i64>().unwrap());
                        solve(level_max, map, level + 1, left) + solve(level_max, map, level + 1, right)
                    },
                    _ => solve(level_max, map,  level + 1, elem * 2024),
                }
            }
        };

        map.insert((level, elem), res);
        return res;
    }

    let mut sum: u64 = 0;
    for elem in input.iter() {
        sum += solve(75, &mut map, 0, *elem);
    }

    println!("{}", sum);
}

fn main() {
    // println!("Part one:");
    // // 188458 too low
    // // 203228
    // part_one("day-11/input.txt");

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
