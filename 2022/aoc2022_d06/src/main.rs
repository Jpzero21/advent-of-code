/*
    Advent of Code 2022 / Day 6: https://adventofcode.com/2022/day/6
    --- Day 6: Tuning Trouble ---
    Q1: How many characters need to be processed before the first start-of-packet
        marker is detected?
    Q2: How many characters need to be processed before the first start-of-message
        marker is detected?
*/

use std::fs::File;
use std::io::Read;

fn main() {
    let mut input_file = File::open("./input.txt").expect("Cannot open input file.");
    let mut line = String::new();
    input_file
        .read_to_string(&mut line)
        .expect("Error reading file content");

    let mut vec = vec![
        line.chars().nth(0).unwrap(),
        line.chars().nth(1).unwrap(),
        line.chars().nth(2).unwrap(),
        line.chars().nth(3).unwrap(),
    ];
    let mut starting_index: usize = 0;
    let mut unique_amount = 4;

    // ------ A1 ------- //
    while contains_duplicate(&vec) {
        move_right(&mut vec, &mut starting_index, &line);
    }
    println!(
        "A1 // {} characters need to be processed before the first start-of-packet  \
        marker is detected",
        starting_index + unique_amount
    );

    // ------ A2 ------- //
    unique_amount = 14;
    vec.clear();
    for i in 0..unique_amount {
        vec.push(line.chars().nth(i).unwrap());
    }
    while contains_duplicate(&vec) {
        move_right(&mut vec, &mut starting_index, &line);
    }
    println!(
        "A2 // {} characters need to be processed before the first start-of-message \
        marker is detected",
        starting_index + unique_amount
    );
}

fn contains_duplicate(vec: &Vec<char>) -> bool {
    for i in 0..vec.len() - 1 {
        if vec[i + 1..].contains(vec.get(i).unwrap()) {
            return true;
        }
    }
    false
}

fn move_right(vec: &mut Vec<char>, index: &mut usize, line: &String) -> () {
    let len = vec.len() - 1;
    for i in 0..len {
        vec[i] = vec[i + 1];
    }
    *index += 1;
    vec[len] = line.chars().nth(*index + len).unwrap();
}

