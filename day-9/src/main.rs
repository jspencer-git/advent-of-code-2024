use std::collections::{BTreeMap, BTreeSet};

use std::fmt::Debug;
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

#[derive(Clone, Copy, PartialEq)]
struct Block {
    id: Option<usize>,
    length: usize,
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.id {
                None => ".".repeat(self.length),
                Some(id) => id.to_string().repeat(self.length),
            }
        )
    }
}

fn part_two(path: &str) {
    let mut input = get_input(path);
    input.retain(|x| x != '\n');

    let original: Vec<usize> = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();

    let mut blocks: BTreeMap<usize, Block> = BTreeMap::new();

    let mut id = 0;
    let mut offset: usize = 0;
    for (idx, elem) in original.iter().enumerate() {
        if idx % 2 == 0 {
            blocks.insert(
                offset,
                Block {
                    id: Some(id),
                    length: *elem,
                },
            );
            id += 1;
        } else {
            blocks.insert(
                offset,
                Block {
                    id: None,
                    length: *elem,
                },
            );
        }
        offset += elem;
    }

    let mut blocks_rev = blocks.clone();
    let blocks_iter = blocks.iter();
    let mut blocks_out: BTreeMap<usize, Block> = BTreeMap::new();
    for (start, block) in blocks_iter {
        match block.id {
            Some(_) => {
                if blocks_out.iter().find(|(_, out_block)| out_block.id.is_some() && out_block.id == block.id ).is_none() {
                  blocks_out.insert(*start, *block);
                }
                continue;
            }
            None => {}
        }

        // Looking at a space
        let mut remaining = block.length as i64;
        let mut next_start = *start;
        for (back_start, back_block) in blocks_rev.iter_mut().rev() {
            if back_block.id.is_none() {
                continue;
            }

            if *back_start < *start {
              continue;
            }

            let remainder = remaining - back_block.length as i64;
            if (remainder) < 0 {
                continue;
            }

            match remainder {
                1.. => {
                    blocks_out.insert(next_start, *back_block);
                    back_block.id = None;
                    next_start += back_block.length;
                    remaining = remainder;
                }
                0 => {
                    blocks_out.insert(next_start, *back_block);
                    back_block.id = None;
                    break;
                }
                ..0 => {
                    panic!()
                }
            }
        }
    }

    println!("{:?}", blocks_out);

    // let mut idx = 0;
    // let mut out_copy: Vec<Option<usize>> = out.clone();

    // println!("{:?}", out);
    // for elem in out.iter_mut() {
    //   if let None = elem {
    //     let mut rev_idx = out_copy.len() - 1;
    //     loop {
    //       match out_copy.get(rev_idx) {
    //         None => {
    //           println!("{:?}", out_copy);
    //           panic!()
    //         }
    //         Some(n) => match n {
    //           None => { rev_idx -= 1; },
    //           Some(n) => {
    //             *elem = Some(*n);
    //             println!("Setting elem {} to {:?}", idx, *elem);
    //             out_copy.remove(rev_idx);
    //             println!("Popping elem {}", rev_idx);
    //             break;
    //           }
    //         },
    //       }
    //     }
    //   }

    //   println!("IDX: {}, {:?}", idx, *elem);
    //   println!("Remaining: {}", out_copy.len());
    //   idx += 1;
    //   if idx > out_copy.len() {
    //     *elem = None;
    //   }
    // }

    // println!("{:?}", out);

    let result = blocks_out
      .iter()
      .filter(|(_, block)| block.id.is_some())
      .fold(0, |acc, (start, block)| {
        let mut sum = 0;
        for i in *start..*start+block.length {
          sum += i * block.id.unwrap();
        }

        acc + sum
      });

    println!("RESULT = {}", result);
}

fn main() {
    // println!("Part one:");
    // part_one("day-9/input.txt");

    println!("Part two:");
    // 10922392917804 too high
    // 6359491814941 correct.
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
