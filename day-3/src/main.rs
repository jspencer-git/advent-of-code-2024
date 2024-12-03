use lazy_static::lazy_static;
use regex::Regex;
use std::{fs::File, io::Read};

fn get_input() -> String {
    let file = File::open("day-3/input.txt").unwrap();
    let mut reader = std::io::BufReader::new(file);

    let mut input_data = String::new();
    reader.read_to_string(&mut input_data).unwrap();

    input_data
}

// Using regex crate.
fn part_one() {
    let input = get_input();

    lazy_static! {
        static ref MUL_REGEX: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    }

    let mut sum: i64 = 0;
    for capture in MUL_REGEX.captures_iter(&input) {
        let a = capture.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let b = capture.get(2).unwrap().as_str().parse::<i64>().unwrap();

        let result = a * b;

        println!(
            "{} -> {} x {} = {}",
            capture.get(0).unwrap().as_str(),
            a,
            b,
            result
        );

        sum += result;
    }

    println!("Final sum = {}", sum);
}

fn part_two() {
    let input = get_input();

    lazy_static! {
        static ref MUL_REGEX: Regex = Regex::new(r"(don't)|(do)|(mul\((\d+),(\d+)\))").unwrap();
    }

    let mut sum: i64 = 0;
    let mut enable = true;
    for capture in MUL_REGEX.captures_iter(&input) {
        match capture.get(0).unwrap().as_str() {
            "don't" => enable = false,
            "do" => enable = true,
            _ => {
                let a = capture.get(4).unwrap().as_str().parse::<i64>().unwrap();
                let b = capture.get(5).unwrap().as_str().parse::<i64>().unwrap();

                let result = a * b;

                println!(
                    "{} -> {} x {} = {} (enabled = {})",
                    capture.get(0).unwrap().as_str(),
                    a,
                    b,
                    result,
                    enable
                );

                if enable {
                    sum += result;
                }
            }
        }
    }

    println!("Final sum = {}", sum);
}

fn main() {
    println!("Part one:");
    part_one();

    println!("Part two:");
    part_two();
}
