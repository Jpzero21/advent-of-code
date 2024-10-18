/*
    Advent of Code 2022 / Day 4: https://adventofcode.com/2022/day/4
    --- Day 4: Camp Cleanup ---
    Q1: In how many assignment pairs does one range fully contain the other?
    Q2: In how many assignment pairs do the ranges overlap?
*/

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_file = File::open("./input.txt").expect("Cannot open input file.");
    let file_reader = BufReader::new(&input_file);

    let re = Regex::new(r"(\d{2})|(\d)").unwrap();

    // ---------- A1 ---------- //
    let mut num_pairs_contained = 0;
    // ---------- A2 ---------- //
    let mut num_pairs_overlapping = 0;

    for l in file_reader.lines() {
        if let Ok(line) = l {
            if line.len() != 0 {
                let mut vec: Vec<i32> = Vec::new();
                for cap in re.captures_iter(&line) {
                    match cap.get(0) {
                        Some(cap) => vec.push(cap.as_str().parse().unwrap()),
                        None => panic!("need four numbers per line"),
                    }
                }
                assert_eq!(vec.len(), 4);
                if one_contains_other(&vec) {
                    num_pairs_contained += 1;
                    num_pairs_overlapping += 1;
                } else if ranges_overlap(&vec) {
                    num_pairs_overlapping += 1;
                }
                vec.clear();
            }
        }
    }

    println!(
        "A1 // There are {} assignment pairs in which one range fully contain the other.",
        num_pairs_contained
    );
    println!(
        "A2 // The ranges of {} assignment pairs overlap",
        num_pairs_overlapping
    );
}

fn one_contains_other(vec: &Vec<i32>) -> bool {
    if (vec[0] <= vec[2] && vec[1] >= vec[3]) || (vec[2] <= vec[0] && vec[3] >= vec[1]) {
        return true;
    }
    false
}

fn ranges_overlap(vec: &Vec<i32>) -> bool {
    if (vec[0] >= vec[2] && vec[0] <= vec[3])
        || (vec[1] >= vec[2] && vec[1] <= vec[3])
        || (vec[2] >= vec[0] && vec[2] <= vec[1])
        || (vec[3] >= vec[0] && vec[3] <= vec[1])
    {
        return true;
    }
    false
}

