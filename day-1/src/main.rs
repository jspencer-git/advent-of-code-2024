use std::{fs::File, io::Read};
use std::collections::HashMap;

fn get_lists() -> (Vec<i64>, Vec<i64>) {
    let file = File::open("day-1/input.txt").unwrap();
    let mut reader = std::io::BufReader::new(file);

    let mut input_data = String::new();
    reader.read_to_string(&mut input_data).unwrap();

    let mut list_a = Vec::new();
    let mut list_b = Vec::new();
    let mut is_a = true;

    for item in input_data.split([' ', '\n']).filter(|x| *x != "") {
        if is_a {
            list_a.push(item.parse::<i64>().unwrap());
        } else {
            let trimmed = item.trim();
            list_b.push(trimmed.parse::<i64>().unwrap());
        }

        is_a = !is_a;
    }

    (list_a, list_b)
}

fn part_one() {
    let (mut list_a, mut list_b) = get_lists();

    list_a.sort();
    list_b.sort();

    let mut sum: u64 = 0;

    for i in 0..list_a.len() {
        sum += (list_a[i] - list_b[i]).abs() as u64;
    }

    println!("{}", sum);
}

fn part_two() {
    let mut freq_map: HashMap<i64, u64> = HashMap::new();

    let (list_a, list_b) = get_lists();

    let mut sum: u64 = 0;

    for item in list_a {
        if let Some(value) = freq_map.get(&item) {
            sum += item as u64 * value;
            continue;
        }

        let mut value: u64 = list_b.iter().filter(|x| **x == item).count() as u64;
        value *= item as u64;
        freq_map.insert(item, value);
        sum += value;
    }

    println!("{}", sum);
}

fn main() {
    println!("Part one:");
    part_one();
    
    println!("Part two:");
    part_two();
}
