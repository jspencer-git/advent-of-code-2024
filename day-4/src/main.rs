use std::collections::VecDeque;
use std::{fs::File, io::Read};

fn get_input(path: &str) -> String {
    let file = File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(file);

    let mut input_data = String::new();
    reader.read_to_string(&mut input_data).unwrap();

    input_data
}

fn get_idx_for_dir(idx: i64, dir: (i64, i64), line_width: i64, max_len: i64) -> Option<i64> {
    let start_line = idx / line_width;

    let idx_next = match dir {
        (0, 0) => unreachable!(),
        (-1, -1) => {
            let next = idx - line_width - 1;
            if next / line_width != (start_line - 1) {
                -1
            } else {
                next
            }
        }
        (0, -1) => idx - line_width,
        (1, -1) => {
            let next = idx - line_width + 1;
            if next / line_width != (start_line - 1) {
                -1
            } else {
                next
            }
        }
        (-1, 0) => {
            let next = idx - 1;
            if next / line_width != start_line {
                -1
            } else {
                next
            }
        }
        (1, 0) => {
            let next = idx + 1;
            if next / line_width != start_line {
                -1
            } else {
                next
            }
        }
        (-1, 1) => {
            let next = idx + line_width - 1;
            if next / line_width != (start_line + 1) {
                -1
            } else {
                next
            }
        }
        (0, 1) => idx + line_width,
        (1, 1) => {
            let next = idx + line_width + 1;
            if next / line_width != (start_line + 1) {
                -1
            } else {
                next
            }
        }
        _ => unreachable!(),
    };

    if idx_next < 0 || idx_next >= max_len {
        return None;
    }

    Some(idx_next)
}

fn solve(mut input: String) {
    let line_width = input.lines().next().unwrap().len() as i64;
    input.retain(|x| x != '\n');

    const DIRS: [(i64, i64); 8] = [
        (-1, -1), // TOP LEFT
        (0, -1),  // TOP
        (1, -1),  // TOP RIGHT
        (-1, 0),  // LEFT
        (1, 0),   // RIGHT
        (-1, 1),  // BOTTOM LEFT
        (0, 1),   // BOTTOM
        (1, 1),   // BOTTOM RIGHT
    ];
    const SEARCH_STR: &str = "XMAS";
    let search_fn = |idx: i64, dir: (i64, i64)| -> bool {
        let mut next_idx = Some(idx);
        for ch in SEARCH_STR.chars() {
            if let Some(nidx) = next_idx {
                let current = input.chars().nth(nidx as usize).unwrap();

                if ch != current {
                    return false;
                }

                next_idx = get_idx_for_dir(nidx, dir, line_width, input.len() as i64);
            } else {
                return false;
            }
        }

        return true;
    };

    let mut sum: i64 = 0;
    for idx in 0..input.len() {
        for dir in DIRS {
            if search_fn(idx as i64, dir) {
                sum += 1;
            }
        }
    }

    println!("SUM {}", sum);
}

fn solve2(mut input: String) {
    let line_width = input.lines().next().unwrap().len() as i64;
    input.retain(|x| x != '\n');

    const DIRS: [(i64, i64); 4] = [
        (-1, -1), // TOP LEFT
        (1, -1),  // TOP RIGHT
        (-1, 1),  // BOTTOM LEFT
        (1, 1),   // BOTTOM RIGHT
    ];

    let search_fn = |idx: i64| -> bool {
        let idxs = [
            Some(idx),                                                      // Center
            get_idx_for_dir(idx, (-1, -1), line_width, input.len() as i64), // TOP LEFT
            get_idx_for_dir(idx, (1, -1), line_width, input.len() as i64),  // TOP RIGHT
            get_idx_for_dir(idx, (-1, 1), line_width, input.len() as i64),  // BOTTOM LEFT
            get_idx_for_dir(idx, (1, 1), line_width, input.len() as i64),   // BOTTOM RIGHT
        ];

        let idxs: Vec<i64> = idxs.iter().filter_map(|x| *x).collect();
        if idxs.len() != 5 {
            return false;
        }

        let current = input.chars().nth(idxs[0] as usize).unwrap();
        if current != 'A' {
            return false;
        }

        let (tl, br) = (
            input.chars().nth(idxs[1] as usize).unwrap(),
            input.chars().nth(idxs[4] as usize).unwrap(),
        );
        match tl {
            'S' => {
                if br != 'M' {
                    return false;
                }
            }
            'M' => {
                if br != 'S' {
                    return false;
                }
            }
            _ => return false,
        }

        let (tr, bl) = (
            input.chars().nth(idxs[2] as usize).unwrap(),
            input.chars().nth(idxs[3] as usize).unwrap(),
        );
        match tr {
            'S' => {
                if bl != 'M' {
                    return false;
                }
            }
            'M' => {
                if bl != 'S' {
                    return false;
                }
            }
            _ => return false,
        }

        return true;
    };

    let mut sum: i64 = 0;
    for idx in 0..input.len() {
        if search_fn(idx as i64) {
            sum += 1;
        }
    }

    println!("SUM {}", sum);
}

// fn solve2(mut input: String) {
//     // RIGHT->LEFT
//     // TOP->BOTTOM
//     // BOTTOM->TOP
//     // DIAG TOP RIGHT BOTTOM LEFT
//     // DIAG BOTTOM LEFT TOP RIGHT
//     // DIAG TOP LEFT BOTTOM RIGHT
//     // DIAG BOTTOM RIGHT TOP LEFT

//     let width = input.lines().next().unwrap().len() as i64;
//     let height = input.lines().count();
//     input.retain(|x| x != '\n');

//     const DIRS: [(i64, i64); 8] = [
//         (-1, -1), // TOP LEFT
//         (0, -1),  // TOP
//         (1, -1),  // TOP RIGHT
//         (-1, 0),  // LEFT
//         (1, 0),   // RIGHT
//         (-1, 1),  // BOTTOM LEFT
//         (0, 1),   // BOTTOM
//         (1, 1),   // BOTTOM RIGHT
//     ];

//     // compare at back of vector, so we need to reverse.
//     let search_str_fwd: Vec<char> = "XMAS".chars().rev().collect();
//     let search_str_rev: Vec<char> = "XMAS".chars().collect();
//     let mut sum: i64 = 0;

//     // LEFT->RIGHT
//     let mut search = search_str_fwd.clone();
//     let mut search_started = false;
//     for (i, ch) in input.chars().step_by(1).enumerate() {
//         if i % width as usize == 0 {
//            println!("Resetting, EOL");

//             search = search_str_fwd.clone();
//             search_started = false;
//         }

//         println!("Checking {} ({}) for {:?}", ch, i, search);

//         let next_search = search.last();
//         match next_search {
//             None => {
//               println!("Found at match at {}", i);

//                 sum += 1;
//                 search = search_str_fwd.clone();
//                 search_started = false;
//                 continue;
//             }
//             Some(search_ch) => {
//                 if ch == *search_ch {
//                     search.pop();
//                     search_started = true;
//                     continue;
//                 }

//                 // If we didn't match, reset
//                 if search_started {
//                     search = search_str_fwd.clone();
//                     search_started = false;
//                 }

//                 if ch == *search_ch {
//                   search.pop();
//                   search_started = true;
//                   continue;
//                 }

//             }
//         }
//     }

//     println!("SUM {}", sum);
// }

fn part_one() {
    let input = get_input("day-4/input.txt");
    solve(input);
}

fn part_two() {
    let input = get_input("day-4/input.txt");
    solve2(input);
}

fn main() {
    println!("Part one:");
    // 2306 INCORRECT
    // 2304 INCORRECT
    // 2297 CORRECT
    // part_one();

    println!("Part two:");
    // 1475 CORRECT
    part_two();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        let input = get_input("example-input.txt");
        solve(input);
    }

    #[test]
    fn test_solve_2() {
        let input = get_input("example-input.txt");
        solve2(input);
    }
}
