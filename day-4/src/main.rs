use std::{fs::File, io::Read};

fn get_input() -> String {
  let file = File::open("day-4/input.txt").unwrap();
  let mut reader = std::io::BufReader::new(file);

  let mut input_data = String::new();
  reader.read_to_string(&mut input_data).unwrap();

  input_data
}

fn part_one() {
  let mut input = get_input();
  let line_width = input.lines().next().unwrap().len();
  input.retain(|x| x != '\n');

  const SEARCH_STR: &str = "XMAS";
  // Brute force search. Won't be winning any prizes for speed.
  fn search(input: &str, line_width: usize, to_find: &str, idx: usize, dir: (i64, i64)) -> i64 {
    println!("Searching for {} at index {}", to_find, idx);
    let to_find_front = to_find.chars().next();
    if to_find_front.is_none() {
      return 1;
    }
    let to_find_front = to_find_front.unwrap();

    let ch = input.chars().nth(idx);

    if ch.is_none() {
      return 0;
    }

    let ch = ch.unwrap();
    if ch == '\n' {
      return 0;
    }

    println!(
      "{} {} {}",
      to_find_front,
      match to_find_front == ch {
        true => "==",
        false => "!=",
      },
      ch
    );

    if ch != to_find_front {
      return 0;
    }

    let mut indices = Vec::new();

    if dir == (0, 0) || dir == (-1, -1) {
      indices.push(((-1, -1), idx.checked_sub(line_width + 1).unwrap_or(idx)));
    }
    if dir == (0, 0) || dir == (0, -1) {
      indices.push(((0, -1), idx.checked_sub(line_width).unwrap_or(idx)));
    }
    if dir == (0, 0) || dir == (1, -1) {
      indices.push(((1, -1), idx.checked_sub(line_width - 1).unwrap_or(idx)));
    }
    if dir == (0, 0) || dir == (-1, 0) {
      indices.push(((-1, 0), idx.checked_sub(1).unwrap_or(idx)));
    }
    if dir == (0, 0) || dir == (1, 0) {
      indices.push(((1, 0), idx.checked_add(1).unwrap_or(idx)));
    }
    if dir == (0, 0) || dir == (-1, 1) {
      indices.push(((-1, 1), idx.checked_add(line_width - 1).unwrap_or(idx)));
    }
    if dir == (0, 0) || dir == (0, 1) {
      indices.push(((0, 1), idx.checked_add(line_width).unwrap_or(idx)));
    }
    if dir == (0, 0) || dir == (1, 1) {
      indices.push(((1, 1), idx.checked_add(line_width + 1).unwrap_or(idx)));
    }

    let mut result: i64 = 0;
    for (next_dir, index) in indices {
      if index == idx {
        continue;
      }
      result += search(input, line_width, to_find.get(1..).unwrap_or(""), index, next_dir);
    }

    println!("Result for idx {} = {}", idx, result);
    result
  }

  let mut sum: i64 = 0;
  for idx in 0..input.len() {
    sum += search(&input, line_width, SEARCH_STR, idx, (0, 0));
  }

  println!("SUM = {}", sum);
}

fn part_two() {}

fn main() {
  println!("Part one:");
  part_one();

  println!("Part two:");
  part_two();
}
