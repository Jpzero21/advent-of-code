use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input_file = File::open("./input.txt").expect("Could not open input file");
    let file_reader = BufReader::new(&input_file);

    let mut total_points = 0;
    let mut number_of_original_cards = 0;
    for l in file_reader.lines() {
        if let Ok(line) = l {
            total_points += get_points(&line);
            number_of_original_cards += 1;
        }
    }

    println!("There were {} total points", total_points);

    let input_file = File::open("./input.txt").expect("Could not open input file");
    let file_reader = BufReader::new(&input_file);

    let mut multiplier: Vec<u32> = std::iter::repeat(1)
        .take(number_of_original_cards)
        .collect::<Vec<_>>();

    let mut current_card: usize = 0;
    for l in file_reader.lines() {
        if let Ok(line) = l {
            for i in 1..=get_win_number(&line) {
                if current_card + i < multiplier.len() {
                    multiplier[current_card + i] += multiplier[current_card];
                }
            }
            current_card += 1;
        }
    }

    let total_scratchcards: u32 = multiplier.iter().sum();
    println!("We end up with {} total scratchcards", total_scratchcards)
}

fn get_win_number(line: &str) -> usize {
    // believe it or not I wrote this all by myself
    let re = Regex::new(r"Card\s{1,}\d{1,}:(?<winning_numbers>[\s{0,}\d{1,}\s{0,}]*)[|](?<card_numbers>[\s{0,}\d{1,}\s{0,}]*)").unwrap();

    let Some(caps) = re.captures(line) else {
        panic!("A line didn't match the regex");
    };

    let winning_numbers: Vec<i32> = caps["winning_numbers"]
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let card_numbers: Vec<i32> = caps["card_numbers"]
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    card_numbers
        .iter()
        .copied()
        .filter(|x| winning_numbers.contains(x))
        .count()
}

fn get_points(line: &str) -> usize {
    let mut remaining = get_win_number(&line);
    if remaining == 0 {
        return 0;
    }

    let mut points = 1;
    remaining -= 1;
    while remaining != 0 {
        points *= 2;
        remaining -= 1;
    }

    points
}
