use std::{collections::VecDeque, fs::File, io::Read};

fn get_input(path: &str) -> String {
    let file = File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(file);

    let mut input_data = String::new();
    reader.read_to_string(&mut input_data).unwrap();
    input_data
}

fn part_one(path: &str) {
    let input = get_input(path);
    let mut calibrations = Vec::new();

    for line in input.lines() {
        let mut split_line = line.split(':');
        let target = split_line.next().unwrap().parse::<i64>().unwrap();
        println!("LINE {}, Target {}", line, target);
        let steps: VecDeque<i64> = split_line
            .next()
            .unwrap()
            .split(" ")
            .filter_map(|x| { x.parse::<i64>().ok() })
            .collect();

        calibrations.push((target, steps));
    }

    // Solve
    fn solve(target: i64, steps: &VecDeque<i64>) -> i64 {
        let mut steps_next = steps.clone();
        let a = steps_next.pop_front().unwrap();
        let b = steps_next.pop_front();

        // Base case
        if b.is_none() {
            if a == target {
                return target
            } else {
                return 0;
            }
        }

        let b = b.unwrap();
        // Add case
        let result_add = a + b;
        steps_next.push_front(result_add);
        if solve(target, &steps_next) == target {
            return target;
        }
        
        steps_next.pop_front();
        // Multiply case
        let result_mul = a * b;
        steps_next.push_front(result_mul);
        if solve(target, &steps_next) == target {
            return target;
        }

        0
    }

    let mut sum: i64 = 0;
    for (target, steps) in calibrations {
        sum += solve(target, &steps);
    }

    println!("{}", sum);
}

fn part_two(path: &str) {
    let input = get_input(path);
    let mut calibrations = Vec::new();

    for line in input.lines() {
        let mut split_line = line.split(':');
        let target = split_line.next().unwrap().parse::<i64>().unwrap();
        println!("LINE {}, Target {}", line, target);
        let steps: VecDeque<i64> = split_line
            .next()
            .unwrap()
            .split(" ")
            .filter_map(|x| { x.parse::<i64>().ok() })
            .collect();

        calibrations.push((target, steps));
    }

    // Solve
    fn solve(target: i64, steps: &VecDeque<i64>) -> i64 {
        let mut steps_next = steps.clone();
        let a = steps_next.pop_front().unwrap();
        let b = steps_next.pop_front();

        // Base case
        if b.is_none() {
            if a == target {
                return target
            } else {
                return 0;
            }
        }

        let b = b.unwrap();
        // Add case
        let result_add = a + b;
        steps_next.push_front(result_add);
        if solve(target, &steps_next) == target {
            return target;
        }
        
        steps_next.pop_front();
        // Multiply case
        let result_mul = a * b;
        steps_next.push_front(result_mul);
        if solve(target, &steps_next) == target {
            return target;
        }

        steps_next.pop_front();
        // Concat case
        let result_cat = (a.to_string() + &b.to_string()).parse::<i64>().unwrap();
        steps_next.push_front(result_cat);
        if solve(target, &steps_next) == target {
            return target;
        }

        0
    }

    let mut sum: i64 = 0;
    for (target, steps) in calibrations {
        sum += solve(target, &steps);
    }

    println!("{}", sum);
}

fn main() {
    println!("Part one:");
    // 5030892084481
    part_one("day-7/input.txt");

    println!("Part two:");
    // 91377448644679
    part_two("day-7/input.txt");
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
