use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solution() {
    let file = File::open("./input.txt").expect("Could not open input file.");
    let file_reader = BufReader::new(&file);

    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut register_value = 1;
    let mut current_cycle = 1;
    let values_to_save = vec![20, 60, 100, 140, 180, 220];

    for l in file_reader.lines() {
        if let Ok(line) = l {
            if line.len() != 0 {
                if line.starts_with("noop") {
                    current_cycle += 1;
                    if values_to_save.contains(&current_cycle) {
                        map.insert(current_cycle, register_value);
                    }
                } else {
                    current_cycle += 1;
                    if values_to_save.contains(&current_cycle) {
                        map.insert(current_cycle, register_value);
                    }
                    register_value += &line[5..].trim().parse().unwrap();
                    current_cycle += 1;
                    if values_to_save.contains(&current_cycle) {
                        map.insert(current_cycle, register_value);
                    }
                }
            }
        }
    }

    let mut signal_strength_sum = 0;
    for n in values_to_save {
        signal_strength_sum += map.get(&n).unwrap() * n;
    }
    println!("------------------ A1 ------------------");
    println!(
        "The sum of the six signal strengths is {}",
        signal_strength_sum
    );
}
