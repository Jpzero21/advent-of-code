use crate::direction::Direction;
use std::io::{BufRead, BufReader};

pub fn extract_word(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    direction: &Direction,
    len: usize,
) -> String {
    (0..len)
        .map(|i| match direction {
            Direction::Top => grid[x - i][y],
            Direction::TopRight => grid[x - i][y + i],
            Direction::TopLeft => grid[x - i][y - i],
            Direction::Right => grid[x][y + i],
            Direction::Left => grid[x][y - i],
            Direction::Bottom => grid[x + i][y],
            Direction::BottomRight => grid[x + i][y + i],
            Direction::BottomLeft => grid[x + i][y - i],
        })
        .collect()
}

pub fn get_valid_directions(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    target: &str,
) -> Vec<Direction> {
    Direction::get_all_directions()
        .iter()
        .filter(|dir| dir.is_valid(grid, x, y, target.len()))
        .copied()
        .collect()
}

pub fn get_grid() -> Vec<Vec<char>> {
    let file = match std::fs::File::open("inputs/2024/d04/input.txt") {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("Could not open input file: {}", e),
    };

    let mut grid: Vec<Vec<char>> = vec![];
    for l in file.lines() {
        if let Ok(line) = l {
            let mut curr = vec![];
            for c in line.chars() {
                curr.push(c);
            }
            grid.push(curr);
        }
    }

    grid
}
