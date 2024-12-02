use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let base_10 = 10;

    // --- Part One ---
    let input_file = File::open("inputs/2023/d01/input.txt").expect("Cannot open input file");
    let file_reader = BufReader::new(&input_file);
    let mut total = 0;
    for l in file_reader.lines() {
        if let Ok(line) = l {
            if line.is_empty() {
                continue;
            }

            let mut first_digit: Option<u32> = None;
            let mut second_digit: u32 = 0;
            for c in line.chars() {
                if !c.is_digit(base_10) {
                    continue;
                }
                if first_digit.is_none() {
                    first_digit = c.to_digit(base_10);
                }
                second_digit = c.to_digit(base_10).expect("Could not parse to int");
            }
            let string_value = format!(
                "{}{}",
                first_digit.ok_or(0).unwrap().to_string(),
                second_digit.to_string()
            );
            total += string_value.parse::<i32>().unwrap();
        }
    }
    println!("The first total was {}", total);

    // --- Part Two ---
    let input_file = File::open("inputs/2023/d01/input.txt").expect("Cannot open input file");
    let file_reader = BufReader::new(&input_file);
    let mut actual_total = 0;
    for l in file_reader.lines() {
        if let Ok(line) = l {
            if line.is_empty() {
                continue;
            }

            let mut first_digit: Option<u32> = None;
            let mut second_digit: u32 = 0;

            for mut i in 0..line.len() {
                if let Some(c) = line.chars().nth(i) {
                    if c.is_digit(base_10) {
                        if first_digit.is_none() {
                            first_digit = c.to_digit(base_10);
                        }
                        second_digit = c.to_digit(base_10).expect("Could not parse to int");
                    } else {
                        if let (Some(c), skip) = starts_with_alpha_digit(&line[i..]) {
                            if first_digit.is_none() {
                                first_digit = Some(c);
                            }
                            second_digit = c;
                            i += skip;
                        }
                    }
                }
            }

            let string_value = format!(
                "{}{}",
                first_digit
                    .ok_or(0)
                    .expect("First digit was None")
                    .to_string(),
                second_digit.to_string()
            );
            actual_total += string_value.parse::<i32>().unwrap();
        }
    }

    println!("The actual total was {}", actual_total);
}

fn starts_with_alpha_digit(str: &str) -> (Option<u32>, usize) {
    return if str.starts_with("one") {
        (Some(1), "one".len())
    } else if str.starts_with("two") {
        (Some(2), "two".len())
    } else if str.starts_with("three") {
        (Some(3), "three".len())
    } else if str.starts_with("four") {
        (Some(4), "four".len())
    } else if str.starts_with("five") {
        (Some(5), "five".len())
    } else if str.starts_with("six") {
        (Some(6), "six".len())
    } else if str.starts_with("seven") {
        (Some(7), "seven".len())
    } else if str.starts_with("eight") {
        (Some(8), "eight".len())
    } else if str.starts_with("nine") {
        (Some(9), "nine".len())
    } else {
        (None, 0)
    };
}
