use crate::direction::Direction;
use crate::utils::extract_word;

mod direction;
mod utils;

fn main() {
    let grid = utils::get_grid();

    let xmas = "XMAS";
    let mut xmas_matches = 0;

    let mas = "MAS";
    let mut mas_matches = 0;

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y].eq(&'X') {
                xmas_matches += search_around(&grid, x, y, xmas);
            }
            if grid[x][y].eq(&'A') && valid_crossword(&grid, x, y, mas) {
                mas_matches += 1
            }
        }
    }

    println!("The word XMAS appears {} times.", xmas_matches);
    assert_eq!(xmas_matches, 2662);

    println!("An X-MAS appears {} times.", mas_matches);
    assert_eq!(mas_matches, 2034);
}

fn search_around(grid: &Vec<Vec<char>>, x: usize, y: usize, target: &str) -> u32 {
    let directions = utils::get_valid_directions(&grid, x, y, target);
    let mut number_of_matches = 0;
    directions.iter().for_each(|dir| {
        if extract_word(&grid, x, y, dir, target.len()).eq(target) {
            number_of_matches += 1
        }
    });

    number_of_matches
}

fn valid_crossword(grid: &Vec<Vec<char>>, x: usize, y: usize, target: &str) -> bool {
    if x == 0 || y == 0 {
        return false;
    }

    let (tr_x, tr_y) = (x - 1, y + 1); // top right starting point
    let (tl_x, tl_y) = (x - 1, y - 1); // top left starting point

    if !Direction::BottomLeft.is_valid(grid, tr_x, tr_y, target.len())
        || !Direction::BottomRight.is_valid(grid, tl_x, tl_y, target.len())
    {
        return false;
    }

    let from_tr = extract_word(&grid, tr_x, tr_y, &Direction::BottomLeft, target.len());
    let from_tl = extract_word(&grid, tl_x, tl_y, &Direction::BottomRight, target.len());

    // can match target or target.rev()
    let target_rev = target.chars().rev().collect::<String>();

    let tr_matches = from_tr.eq(target) || from_tr.eq(&target_rev);
    let tl_matches = from_tl.eq(target) || from_tl.eq(&target_rev);

    tr_matches && tl_matches
}
