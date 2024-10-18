use std::cmp::min;
use std::collections::HashMap;

type Node = (usize, usize);
type Grid = Vec<Vec<u8>>;
type Map = HashMap<Node, i32>;

pub fn solution(grid: &Grid, end_position: Node) {
    let mut visited: Map = HashMap::new();
    let min = find_first_a(grid, end_position, 0, &mut visited);
    println!(
        "A2 // The fewest steps required to move from any square a to E is {}",
        min
    );
}

fn find_first_a(grid: &Grid, node: Node, current_len: i32, visited: &mut Map) -> i32 {
    if !visited.contains_key(&node) || visited.get(&node).unwrap() > &current_len {
        visited.insert(node, current_len);
    } else {
        return i32::MAX;
    }

    let (row, col) = node;
    if grid[row][col] == 'a' as u8 {
        return current_len;
    }

    let mut min_len: i32 = i32::MAX;
    let mut next_nodes = Vec::new();
    if row != 0 {
        next_nodes.push((row - 1, col));
    }
    if col != 0 {
        next_nodes.push((row, col - 1));
    }
    if row < grid.len() - 1 {
        next_nodes.push((row + 1, col));
    }
    if col < grid[0].len() - 1 {
        next_nodes.push((row, col + 1));
    }

    for next_node in next_nodes {
        min_len = min(
            min_len,
            check_next(grid, node, next_node, current_len, visited),
        );
    }
    min_len
}

fn check_next(grid: &Grid, prev: Node, next: Node, current_len: i32, visited: &mut Map) -> i32 {
    // the order of the difference here is the opposite of that in the previous part
    let difference: i32 = grid[prev.0][prev.1] as i32 - grid[next.0][next.1] as i32;
    if difference > 1 {
        return i32::MAX;
    }

    return if visited.contains_key(&next) {
        if visited.get(&next).unwrap() > &(current_len + 1) {
            find_first_a(&grid, next, current_len + 1, visited)
        } else {
            i32::MAX
        }
    } else {
        find_first_a(&grid, next, current_len + 1, visited)
    };
}
