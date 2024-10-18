/*
    Advent of Code 2022 / Day 12: https://adventofcode.com/2022/day/12
    --- Day 12: Hill Climbing Algorithm ---
    Q1: What is the fewest steps required to move from your current position to the location
        that should get the best signal?
    Q2: What is the fewest steps required to move starting from any square with elevation a
        to the location that should get the best signal?
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

mod d12_a1;
mod d12_a2;

fn main() {
    let file = File::open("./input.txt").expect("Could not open input file.");
    let file = BufReader::new(&file);

    let mut grid: Vec<Vec<u8>> = Vec::new();
    let (start_position, end_position) = init_grid(&mut grid, file);
    let grid = grid;

    d12_a1::solution(&grid, start_position, end_position);
    d12_a2::solution(&grid, end_position);
}

fn init_grid(grid: &mut Vec<Vec<u8>>, file: BufReader<&File>) -> ((usize, usize), (usize, usize)) {
    let mut start_position: (usize, usize) = (0, 0);
    let mut end_position: (usize, usize) = (0, 0);
    let (mut found_start, mut found_end) = (false, false);

    for l in file.lines() {
        if let Ok(mut line) = l {
            if line.len() != 0 {
                if !found_start && line.contains('S') {
                    found_start = true;
                    let row = grid.len();
                    let col = line.find('S').unwrap();
                    start_position = (row, col);
                    line = line.replace("S", "a");
                }
                if !found_end && line.contains('E') {
                    found_end = true;
                    let row = grid.len();
                    let col = line.find('E').unwrap();
                    end_position = (row, col);
                    line = line.replace("E", "z");
                }
                let line = Vec::from(line);
                grid.push(line);
            }
        }
    }
    (start_position, end_position)
}
