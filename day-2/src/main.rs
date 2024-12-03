use std::{fs::File, io::Read, thread, time::Duration};

fn get_inputs() -> Vec<Vec<i64>> {
    let file = File::open("day-2/input.txt").unwrap();
    let mut reader = std::io::BufReader::new(file);

    let mut input_data = String::new();
    reader.read_to_string(&mut input_data).unwrap();

    let mut inputs = Vec::new();
    for item in input_data.split(['\n']) {
        if item == "" {
            continue;
        }

        let report = item
            .split(' ')
            .filter_map(|x| match x.parse::<i64>() {
                Ok(res) => Some(res),
                Err(_) => None,
            })
            .collect();

        inputs.push(report);
    }

    inputs
}

fn part_one() {
    let inputs = get_inputs();

    let mut safe: u64 = 0;
    for report in inputs {
        let mut report_iter = report.into_iter();
        let mut last_level = report_iter.next().unwrap();
        let mut safe_dir_pos = None;

        let mut is_safe = true;
        for level in report_iter {
            let diff = level - last_level;

            if diff.abs() > 3 || diff.abs() < 1 {
                is_safe = false;
                break;
            }

            if let Some(is_pos) = safe_dir_pos {
                if diff.is_positive() != is_pos {
                    is_safe = false;
                    break;
                }
            } else {
                safe_dir_pos = Some(diff.is_positive());
            }

            last_level = level;
        }

        if is_safe {
            safe += 1;
        }
    }

    println!("{}", safe);
}

fn part_two() {
    let inputs = get_inputs();

    let check_levels = |diff_a: i64, diff_b: i64| {
        if diff_a.abs() > 3 || diff_a.abs() < 1 {
            return false;
        }

        if diff_b.abs() > 3 || diff_b.abs() < 1 {
            return false;
        }

        if diff_a.signum() != diff_b.signum() {
            return false;
        }

        true
    };

    let safe_count = brute_force(inputs);

    println!("{}", safe_count);
}

// Brute force approach. Could use this to generate test cases for a more efficient algo.
// However, after 6 wrong attempts its time to move on...
fn brute_force(reports: Vec<Vec<i64>>) -> i64 {
    let check_report = |report: &Vec<i64>| -> bool {
        let report_diffs: Vec<i64> = report
            .windows(2)
            .map(|w| {
                let [x, y] = w else { unreachable!() };
                y - x
            })
            .collect();

        report_diffs.windows(2).all(|w| {
            let [diff_a, diff_b] = w else { unreachable!() };
            if diff_a.abs() > 3 || diff_a.abs() < 1 {
                return false;
            }

            if diff_b.abs() > 3 || diff_b.abs() < 1 {
                return false;
            }

            if diff_a.signum() != diff_b.signum() {
                return false;
            }

            true
        })
    };

    let mut safe: i64 = 0;
    for report in reports {
        println!("REPORT: {:?}", report);

        let mut ok = check_report(&report);

        println!("OK = {}", ok);

        let mut remove_idx = 0;
        while !ok && remove_idx < report.len() {
            
            let mut new_report: Vec<i64> = report.clone();
            new_report.remove(remove_idx);
            println!("SUB REPORT: {:?}", new_report);

            ok = check_report(&new_report);
            println!("OK = {}", ok);
            
            remove_idx += 1;
        }

        if ok {
            safe += 1;
        }

        println!("SAFE = {}\n", ok);
    }

    safe
}

// Attempt 1
fn try_solve_1(report_diffs: Vec<i64>, sign: i64) -> bool {
    let mut fail = false;
    let mut carry: i64 = 0;
    let mut err_last = false;
    let mut lives = 1;
    for w in report_diffs.windows(2) {
        let [x, mut y] = w else { unreachable!() };

        // If we had an error previously, sum the diffs to see if removing it
        // can make the rest of the sequence valid. (doesn't work for all cases.)
        if err_last {
            err_last = false;
            y += carry;
        } else {
            // Case of X failed. This should only happen at the start, or if the
            // previous window had an error.
            if x.abs() > 3 || x.abs() < 1 || x.signum() != sign {
                lives -= 1;

                if lives < 0 {
                    // 0 lives left and we failed another check.
                    fail = true;
                    break;
                }

                // Edge case. Beginning of sequence. Just drop the first diff and carry on.
                continue;
            }
        }

        // Case of Y failed.
        if y.abs() > 3 || y.abs() < 1 || y.signum() != sign {
            lives -= 1;

            if lives < 0 {
                // 0 lives left and we failed another check.
                fail = true;
                break;
            }

            // Y check failed
            if !err_last {
                err_last = true;
                carry = y;
                continue;
            }
        }
    }

    fail
}

fn main() {
    println!("Part one:");
    part_one();

    println!("Part two:");
    part_two();
}
