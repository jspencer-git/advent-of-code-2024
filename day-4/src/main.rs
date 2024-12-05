use std::{fs::File, io::Read};

fn get_input() -> String {
    let file = File::open("day-4/input.txt").unwrap();
    let mut reader = std::io::BufReader::new(file);

    let mut input_data = String::new();
    reader.read_to_string(&mut input_data).unwrap();

    input_data
}

fn get_idx_for_dir(idx: i64, dir: (i64, i64), line_width: i64, max_len: i64) -> Option<i64> {
    let idx_next = match dir {
        (0, 0) => unreachable!(),
        (-1, -1) => idx - line_width - 1,
        (0, -1) => idx - line_width,
        (1, -1) => idx - line_width + 1,
        (-1, 0) => idx - 1,
        (1, 0) => idx + 1,
        (-1, 1) => idx + line_width - 1,
        (0, 1) => idx + line_width,
        (1, 1) => idx + line_width + 1,
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
    let search_fn = |first: char, idx: i64, dir: (i64, i64)| -> bool {
      let mut current = first;
      let mut next_idx = idx;
      for ch in SEARCH_STR.chars() {
        if ch != current {
          return false;
        }

        let next_idx_opt = get_idx_for_dir(next_idx, dir, line_width, input.len() as i64);
        if next_idx_opt.is_none() {
          if ch == SEARCH_STR.chars().rev().next().unwrap() {
            return true;
          }

          return false;
        }

        next_idx = next_idx_opt.unwrap();
        current = input.chars().nth(next_idx as usize).unwrap();
      }

      return true;
    };

    let mut sum: i64 = 0;
    for idx in 0..input.len() {
      for dir in DIRS {
        let first = input.chars().nth(idx).unwrap();
        if search_fn(first, idx as i64, dir) {
          sum += 1;
        }
      }
    }

    println!("SUM {}", sum);
}

fn part_one() {
    let input = get_input();
    solve(input);

    // const SEARCH_STR: &str = "XMAS";
    // // Brute force search. Won't be winning any prizes for speed.
    // fn search(input: &str, line_width: usize, to_find: &str, idx: usize, dir: (i64, i64)) -> i64 {
    //     println!("Searching for {} at index {}", to_find, idx);
    //     let to_find_front = to_find.chars().next();
    //     if to_find_front.is_none() {
    //         return 1;
    //     }
    //     let to_find_front = to_find_front.unwrap();

    //     let ch = input.chars().nth(idx);

    //     if ch.is_none() {
    //         return 0;
    //     }

    //     let ch = ch.unwrap();
    //     if ch == '\n' {
    //         return 0;
    //     }

    //     println!(
    //         "{} {} {}",
    //         to_find_front,
    //         match to_find_front == ch {
    //             true => "==",
    //             false => "!=",
    //         },
    //         ch
    //     );

    //     if ch != to_find_front {
    //         return 0;
    //     }

    //     let mut indices = Vec::new();

    //     if dir == (0, 0) || dir == (-1, -1) {
    //         indices.push(((-1, -1), idx.checked_sub(line_width + 1).unwrap_or(idx)));
    //     }
    //     if dir == (0, 0) || dir == (0, -1) {
    //         indices.push(((0, -1), idx.checked_sub(line_width).unwrap_or(idx)));
    //     }
    //     if dir == (0, 0) || dir == (1, -1) {
    //         indices.push(((1, -1), idx.checked_sub(line_width - 1).unwrap_or(idx)));
    //     }
    //     if dir == (0, 0) || dir == (-1, 0) {
    //         indices.push(((-1, 0), idx.checked_sub(1).unwrap_or(idx)));
    //     }
    //     if dir == (0, 0) || dir == (1, 0) {
    //         indices.push(((1, 0), idx.checked_add(1).unwrap_or(idx)));
    //     }
    //     if dir == (0, 0) || dir == (-1, 1) {
    //         indices.push(((-1, 1), idx.checked_add(line_width - 1).unwrap_or(idx)));
    //     }
    //     if dir == (0, 0) || dir == (0, 1) {
    //         indices.push(((0, 1), idx.checked_add(line_width).unwrap_or(idx)));
    //     }
    //     if dir == (0, 0) || dir == (1, 1) {
    //         indices.push(((1, 1), idx.checked_add(line_width + 1).unwrap_or(idx)));
    //     }

    //     let mut result: i64 = 0;
    //     for (next_dir, index) in indices {
    //         if index == idx {
    //             continue;
    //         }
    //         result += search(
    //             input,
    //             line_width,
    //             to_find.get(1..).unwrap_or(""),
    //             index,
    //             next_dir,
    //         );
    //     }

    //     println!("Result for idx {} = {}", idx, result);
    //     result
    // }

    // let mut sum: i64 = 0;
    // for idx in 0..input.len() {
    //     sum += search(&input, line_width, SEARCH_STR, idx, (0, 0));
    // }

    // println!("SUM = {}", sum);
}

fn part_two() {}

fn main() {
    println!("Part one:");
    part_one();

    println!("Part two:");
    // 2306 INCORRECT
    part_two();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
      let input = "XMAS";
      solve(input.into());
    }
}