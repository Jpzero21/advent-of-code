/*
    Advent of Code 2022 / Day 5: https://adventofcode.com/2022/day/5
    --- Day 5: Supply Stacks ---
    Q1: After the rearrangement procedure completes, what crate ends up on top of each stack?
    Q2: After the rearrangement procedure completes, what crate ends up on top of each stack?
*/

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut stacks1: Vec<Vec<char>> = Vec::new();
    // is this cheating?
    stacks1.push(Vec::new());
    stacks1.push(Vec::from(['B', 'L', 'D', 'T', 'W', 'C', 'F', 'M']));
    stacks1.push(Vec::from(['N', 'B', 'L']));
    stacks1.push(Vec::from(['J', 'C', 'H', 'T', 'L', 'V']));
    stacks1.push(Vec::from(['S', 'P', 'J', 'W']));
    stacks1.push(Vec::from(['Z', 'S', 'C', 'F', 'T', 'L', 'R']));
    stacks1.push(Vec::from(['W', 'D', 'G', 'B', 'H', 'N', 'Z']));
    stacks1.push(Vec::from(['F', 'M', 'S', 'P', 'V', 'G', 'C', 'N']));
    stacks1.push(Vec::from(['W', 'Q', 'R', 'J', 'F', 'V', 'C', 'Z']));
    stacks1.push(Vec::from(['R', 'P', 'M', 'L', 'H']));
    let mut stacks2 = stacks1.clone();

    let re = Regex::new(r"(\d{2})|(\d)").unwrap();

    let input_file = File::open("./move_operations.txt").expect("Cannot open input file.");
    let file_reader = BufReader::new(&input_file);

    for l in file_reader.lines() {
        if let Ok(line) = l {
            if line.len() != 0 {
                let mut op_values: Vec<i32> = Vec::new();
                for cap in re.captures_iter(&line) {
                    match cap.get(0) {
                        Some(caps) => op_values.push(caps.as_str().parse().unwrap()),
                        None => panic!("input is invalid"),
                    }
                }
                assert_eq!(op_values.len(), 3);
                let num_to_move = op_values[0];
                let origin = op_values[1] as usize;
                let destination = op_values[2] as usize;
                // ---------- A1 ---------- //
                for _i in 1..=num_to_move {
                    let character = match stacks1[origin].pop() {
                        Some(character) => character,
                        None => break,
                    };
                    stacks1[destination].push(character);
                }
                // ---------- A2 ---------- //
                let mut temp_vec: Vec<char> = Vec::new();
                for _i in 1..=num_to_move {
                    let character = match stacks2[origin].pop() {
                        Some(character) => character,
                        None => break,
                    };
                    temp_vec.push(character);
                }
                for _i in 0..temp_vec.len() {
                    let character = temp_vec.pop().unwrap();
                    stacks2[destination].push(character);
                }
            }
        }
    }

    print!("A1 // After the rearrangement procedure completes, the crates that end up on top of each stack are ");
    for i in 1..stacks1.len() {
        match stacks1[i].pop() {
            Some(character) => print!("{}", character),
            None => print!(" "),
        }
    }
    println!();

    print!("A2 // After the rearrangement procedure completes, the crates that end up on top of each stack are ");
    for i in 1..stacks2.len() {
        match stacks2[i].pop() {
            Some(character) => print!("{}", character),
            None => print!(" "),
        }
    }
    println!();
}
