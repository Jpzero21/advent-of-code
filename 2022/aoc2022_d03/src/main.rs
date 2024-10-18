/*
    Advent of Code 2022 / Day 3: https://adventofcode.com/2022/day/3
    --- Day 3: Rucksack Reorganization ---
    Q1: Find the item type that appears in both compartments of each rucksack.
        What is the sum of the priorities of those item types?
    Q2: Find the item type that corresponds to the badges of each three-Elf group.
        What is the sum of the priorities of those item types?
*/

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_file = File::open("./input.txt").expect("Cannot open input file.");
    let file_reader = BufReader::new(&input_file);

    // ---------- A1 variables ---------- //
    let mut priority_sum = 0;
    let mut rucksack_contents = HashSet::new();
    // ---------- A2 variables ---------- //
    let mut group_vecs: Vec<String> = Vec::new();
    let mut group_sets: Vec<HashSet<char>> = Vec::new();
    for _i in 0..=2 {
        group_sets.push(HashSet::new());
    }
    let mut group_sum = 0;

    for l in file_reader.lines() {
        if let Ok(line) = l {
            if line.len() != 0 {
                // ---------- A2 ---------- //
                group_vecs.push(line.clone());
                if group_vecs.len() == 3 {
                    for i in 0..=1 {
                        for c in group_vecs[i].chars() {
                            group_sets[i].insert(c);
                        }
                    }
                    for c in group_vecs[2].chars() {
                        if group_sets[0].contains(&c) && group_sets[1].contains(&c) {
                            if c >= 'a' && c <= 'z' {
                                group_sum += c as u32 - 96;
                            } else {
                                group_sum += c as u32 - 38;
                            }
                            break;
                        }
                    }
                    group_vecs = Vec::new();
                    group_sets = Vec::new();
                    for _i in 0..=2 {
                        group_sets.push(HashSet::new());
                    }
                }
                // ---------- A1 ---------- //
                for c in line[..line.len() / 2].chars() {
                    rucksack_contents.insert(c);
                }
                for c in line[line.len() / 2..].chars() {
                    if rucksack_contents.contains(&c) {
                        // map the characters from their ascii values to the ones needed
                        if c >= 'a' && c <= 'z' {
                            priority_sum += c as u32 - 96;
                        } else {
                            priority_sum += c as u32 - 38;
                        }
                        rucksack_contents = HashSet::new();
                        continue;
                    }
                }
            }
        }
    }

    println!(
        "A1 // The sum of priorities of the item types that appear in both compartments \
        of each rucksack is {}",
        priority_sum
    );

    println!(
        "A2 // The sum of priorities of the item types that correspond to the badges \
        of each three-Elf group is {}",
        group_sum
    );
}
