use std::cmp::max;
use std::collections::HashSet;

enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

struct Side {
    direction: Direction,
    row_start: usize,
    row_end: usize,
    col_start: usize,
    col_end: usize,
}

pub fn solution(grid: &Vec<Vec<i32>>) -> i32 {
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    let mut number_visible = 0;

    let mut sides: Vec<Side> = Vec::new();
    sides.push(Side::new(Direction::Top, &grid));
    sides.push(Side::new(Direction::Bottom, &grid));
    sides.push(Side::new(Direction::Left, &grid));
    sides.push(Side::new(Direction::Right, &grid));
    for side in sides {
        visit_from(side, &grid, &mut set, &mut number_visible);
    }
    return number_visible;
}

fn visit_from(side: Side, grid: &Vec<Vec<i32>>, set: &mut HashSet<(usize, usize)>, vis: &mut i32) {
    match side.direction {
        Direction::Top | Direction::Bottom => {
            for col in side.col_start..=side.col_end {
                let mut row = side.row_start;
                let mut max_tree = -1;
                loop {
                    let current_tree = grid[row][col];
                    if current_tree > max_tree && !set.contains(&(row, col)) {
                        set.insert((row, col));
                        *vis += 1;
                    }
                    max_tree = max(max_tree, current_tree);
                    if row == side.row_end {
                        break;
                    }
                    side.move_next(&mut row);
                }
            }
        }

        Direction::Left | Direction::Right => {
            for row in side.row_start..=side.row_end {
                let mut col = side.col_start;
                let mut max_tree = -1;
                loop {
                    let current_tree = grid[row][col];
                    if current_tree > max_tree && !set.contains(&(row, col)) {
                        set.insert((row, col));
                        *vis += 1;
                    }
                    max_tree = max(max_tree, current_tree);
                    if col == side.col_end {
                        break;
                    }
                    side.move_next(&mut col);
                }
            }
        }
    }
}

impl Side {
    fn new(direction: Direction, grid: &Vec<Vec<i32>>) -> Side {
        let (rows, cols) = (grid.len() - 1, grid[0].len() - 1);

        let mut side: Side = Side {
            direction,
            row_start: 0,
            row_end: rows,
            col_start: 0,
            col_end: cols,
        };

        match side.direction {
            Direction::Top => {
                side.row_start = 0;
                side.row_end = rows;
            }
            Direction::Bottom => {
                side.row_start = rows;
                side.row_end = 0;
            }
            Direction::Left => {
                side.col_start = 0;
                side.col_end = cols;
            }
            Direction::Right => {
                side.col_start = cols;
                side.col_end = 0;
            }
        }
        return side;
    }

    fn move_next(&self, position: &mut usize) {
        match self.direction {
            Direction::Top | Direction::Left => {
                *position += 1;
            }
            Direction::Bottom | Direction::Right => {
                *position -= 1;
            }
        }
    }
}
