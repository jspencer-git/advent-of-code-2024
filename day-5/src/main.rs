use std::collections::{HashMap, HashSet};
use std::{fs::File, io::Read};

fn get_input(path: &str) -> (String, String) {
    let file = File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(file);

    let mut input_data = String::new();
    reader.read_to_string(&mut input_data).unwrap();

    let inputs: Vec<&str> = input_data.split("\n\n").collect();
    assert!(inputs.len() == 2);

    (String::from(inputs[0]), String::from(inputs[1]))
}

fn part_one(path: &str) {
    let (rules, updates) = get_input(path);
    let rules: Vec<(i64, i64)> = rules
        .lines()
        .map(|l| {
            let a_b: Vec<i64> = l
                .trim()
                .split("|")
                .map(|x| x.parse::<i64>().unwrap())
                .collect();

            assert!(a_b.len() == 2);

            (a_b[0], a_b[1])
        })
        .collect();

    let mut rule_map: HashMap<i64, HashSet<i64>> = HashMap::new();
    for (a, b) in rules {
        if !rule_map.contains_key(&b) {
            rule_map.insert(b, HashSet::new());
        }

        rule_map.get_mut(&b).unwrap().insert(a);
    }

    // For each entry B in rule map, every item in set A must have been printed already
    println!("RULES ARE: {:?}", rule_map);

    let mut sum: i64 = 0;
    for update in updates.lines() {
        println!("Update: {}", update);
        let update_items: Vec<i64> = update
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        
        let mut ok = true;
        let mut all: HashSet<i64> = HashSet::new();
        update_items.iter().for_each(|x| {all.insert(*x);} );

        let mut used: HashSet<i64> = HashSet::new();

        for item in update_items.iter() {
            if let Some(r) = rule_map.get(item) {
                if !r.iter().all(|x| {
                    !all.contains(x) ||
                    used.contains(x)
                }) {
                    println!("Check for {} failed. {:?} does not contain all of {:?}", item, used, r);
                    ok = false;
                    break;
                }
            }
            
            println!("Check for {} succeeded", item);
            used.insert(*item);
        }

        if ok {
            let mid = update_items.get(update_items.len() / 2);
            if let Some(i) = mid {
                println!("Update {:?} is OK. Mid is {}", update, i);
                sum += i;
            }
        }
    }

    println!("Sum = {}", sum);
}

fn part_two() {}

fn main() {
    println!("Part one:");
    part_one("day-5/input.txt");

    println!("Part two:");
    part_two();
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
}
