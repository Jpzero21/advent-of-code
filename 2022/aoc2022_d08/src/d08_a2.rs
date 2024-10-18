use std::cmp::max;

pub fn solution(grid: &Vec<Vec<i32>>) -> u64 {
    let mut max_score = 0;
    for row in 1..grid.len() - 1 {
        for col in 1..grid[0].len() - 1 {
            max_score = max(max_score, get_scenic_score(&grid, row, col));
        }
    }
    return max_score;
}

fn get_scenic_score(grid: &Vec<Vec<i32>>, r: usize, c: usize) -> u64 {
    let current_tree = grid[r][c];
    let (mut above, mut below, mut left, mut right) = (0, 0, 0, 0);

    for r_var in (0..r).rev() {
        above += 1;
        if grid[r_var][c] >= current_tree {
            break;
        }
    }
    for r_var in r + 1..grid.len() {
        below += 1;
        if grid[r_var][c] >= current_tree {
            break;
        }
    }
    for c_var in (0..c).rev() {
        left += 1;
        if grid[r][c_var] >= current_tree {
            break;
        }
    }
    for c_var in c + 1..grid[0].len() {
        right += 1;
        if grid[r][c_var] >= current_tree {
            break;
        }
    }

    return above * below * left * right;
}
