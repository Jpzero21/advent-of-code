use std::fs::File;
use std::io::{BufRead, BufReader};

mod d08_a1;
mod d08_a2;

fn main() {
    let file = File::open("inputs/2022/d08/input.txt").expect("Error opening input file");
    let file_reader = BufReader::new(&file);

    let mut grid: Vec<Vec<i32>> = Vec::new();
    for l in file_reader.lines() {
        if let Ok(line) = l {
            if line.len() != 0 {
                let mut vec: Vec<i32> = Vec::new();
                for c in line.chars() {
                    vec.push(c.to_string().parse().unwrap());
                }
                grid.push(vec);
            }
        }
    }

    println!(
        "A1 // {} trees are visible from outside the grid.",
        d08_a1::solution(&grid)
    );

    println!(
        "A2 // The highest scenic score possible for any tree is {}.",
        d08_a2::solution(&grid)
    );
}
