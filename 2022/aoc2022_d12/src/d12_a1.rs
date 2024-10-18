use std::collections::HashMap;

type Node = (usize, usize);
type Grid = Vec<Vec<u8>>;
type Map = HashMap<Node, i32>;

pub fn solution(grid: &Grid, start_position: Node, end_position: Node) {
    let mut visited: Map = HashMap::new();
    traverse_tree(&grid, start_position, 0, &mut visited);

    if visited.contains_key(&end_position) {
        println!(
            "A1 // The fewest number of steps required to move from S to E is {}",
            visited.get(&end_position).unwrap()
        );
    } else {
        println!("Could not reach the the end from starting position");
    }
}

fn traverse_tree(grid: &Grid, node: Node, current_len: i32, visited: &mut Map) {
    // if node is not visited yet, or this is the shortest road to this node, proceed
    if !visited.contains_key(&node) || visited.get(&node).unwrap() > &current_len {
        visited.insert(node, current_len);
    } else {
        return;
    }

    let (row, col) = node;
    if row != 0 {
        let above = (row - 1, col);
        check_next(grid, node, above, current_len, visited);
    }
    if row < grid.len() - 1 {
        let below = (row + 1, col);
        check_next(grid, node, below, current_len, visited);
    }
    if col != 0 {
        let left = (row, col - 1);
        check_next(grid, node, left, current_len, visited);
    }
    if col < grid[0].len() - 1 {
        let right = (row, col + 1);
        check_next(grid, node, right, current_len, visited);
    }
}

fn check_next(grid: &Grid, previous: Node, next: Node, current_len: i32, visited: &mut Map) {
    let difference: i32 = grid[next.0][next.1] as i32 - grid[previous.0][previous.1] as i32;
    if difference > 1 {
        return;
    }

    if visited.contains_key(&next) {
        if visited.get(&next).unwrap() > &(current_len + 1) {
            traverse_tree(&grid, next, current_len + 1, visited);
        }
    } else {
        traverse_tree(&grid, next, current_len + 1, visited);
    }
}
