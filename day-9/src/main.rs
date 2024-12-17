use utils::*;

// let mut input = get_input(path);
// input.retain(|x| x != '\n');

// let original: Vec<usize> = input
//   .chars()
//   .map(|x| x.to_digit(10).unwrap() as usize)
//   .collect();

// let mut out: Vec<(i64, usize)> = Vec::new();

// println!("Original list: {:?}", original);
// let mut id = 0;
// let mut is_block = true;
// for item in original.iter() {
//   if is_block {
//     print!("{}", id.to_string().repeat(*item));
//     id += 1;
//   } else {
//     print!("{}", ".".repeat(*item));
//   }

//   is_block = !is_block;
// }

// let mut blocks: Vec<(usize, usize)> = original.clone().into_iter().step_by(2).enumerate().collect();
// let mut blocks_rev: Vec<(usize, usize)> = blocks.clone().into_iter().rev().collect();
// let spaces: Vec<usize> = original.clone().into_iter().skip(1).step_by(2).collect();

// let mut blocks_zip: Vec<(usize, usize, usize)> = Vec::new();
// let mut space_idx = 1;
// for space in spaces.into_iter() {
//   let mut remaining = space;
//   let back = blocks_rev.pop();
//   if back.is_none() {
//     break;
//   }
//   let back = back.unwrap();

//   let res = remaining as i64 - back.1 as i64;
//   match remaining {
//     0.. => blocks_zip.push(space_idx, back.i)
//   }
// }

fn part_one(path: &str) {
  let mut input = get_input(path);
  input.retain(|x| x != '\n');

  let original: Vec<usize> = input
    .chars()
    .map(|x| x.to_digit(10).unwrap() as usize)
    .collect();

  let mut out: Vec<Option<usize>> = Vec::new();

  let mut id = 0;
  for (idx, elem) in original.iter().enumerate() {
    if idx % 2 == 0 {
      for _ in 0..*elem {
        out.push(Some(id));
      }
      id += 1;
    } else {
      for _ in 0..*elem {
        out.push(None);
      }
    }
  }

  let mut idx = 0;
  let mut out_copy: Vec<Option<usize>> = out.clone();

  println!("{:?}", out);
  for elem in out.iter_mut() {
    if let None = elem {
      loop {
        match out_copy.pop() {
          None => {
            println!("{:?}", out_copy);
            panic!()
          }
          Some(n) => match n {
            None => continue,
            Some(n) => {
              *elem = Some(n);
              println!("Setting elem {} to {:?}", idx, *elem);
              break;
            }
          },
        }
      }
    }

    println!("IDX: {}, {:?}", idx, *elem);
    println!("Remaining: {}", out_copy.len());
    idx += 1;
    if idx > out_copy.len() {
      *elem = None;
    }
  }

  println!("{:?}", out);

  let result = out
    .iter()
    .filter_map(|x| *x)
    .enumerate()
    .fold(0, |acc, (idx, x)| acc + (idx * x));

  println!("RESULT = {}", result);
}

fn part_two(path: &str) {}

fn main() {
  println!("Part one:");
  part_one("day-9/input.txt");

  println!("Part two:");
  part_two("day-9/input.txt");
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
