/*
    Advent of Code 2022 / Day 9: https://adventofcode.com/2022/day/9
    --- Day 9: Rope Bridge ---
    Q1: Simulate your complete hypothetical series of motions.
        How many positions does the tail of the rope visit at least once?
    Q2: Simulate your complete series of motions on a larger rope with ten knots.
        How many positions does the tail of the rope visit at least once?
*/

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Direction {
    U,
    D,
    L,
    R,
}

struct Move {
    direction: Direction,
    amount: u32,
}

struct Knot {
    x: i32,
    y: i32,
}

fn main() {
    // ------- A1 variables ------- //
    let mut h = Knot { x: 0, y: 0 };
    let mut t = Knot { x: 0, y: 0 };
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    set.insert((0, 0));
    // ---------------------------- //

    // ------- A2 variables ------- //
    let mut rope: Vec<Knot> = Vec::new();
    for _i in 0..10 {
        rope.push(Knot { x: 0, y: 0 })
    }
    let mut set_two: HashSet<(i32, i32)> = HashSet::new();
    set_two.insert((0, 0));
    // ---------------------------- //

    let file = File::open("./input.txt").expect("Could not open input file.");
    let file_reader = BufReader::new(&file);

    for l in file_reader.lines() {
        if let Ok(line) = l {
            if line.len() != 0 {
                let current_move: Move = Move::new(
                    line.chars().nth(0).unwrap(),
                    line[2..].trim().parse().unwrap(),
                );
                h.make_move(&current_move, &mut t, &mut set);
                make_move(&mut rope, &current_move, &mut set_two);
            }
        }
    }

    println!(
        "A1 // The tail of the rope visits {} positions at least once.",
        set.len()
    );
    println!(
        "A2 // The tail of the ten knot rope visits {} positions at least once.",
        set_two.len()
    );
}

impl Move {
    fn new(letter: char, amount: u32) -> Move {
        Move {
            direction: match letter {
                'U' => Direction::U,
                'D' => Direction::D,
                'L' => Direction::L,
                'R' => Direction::R,
                _ => panic!("invalid argument for Direction::new"),
            },
            amount,
        }
    }
}

// ---------- A1 ---------- //
impl Knot {
    fn make_move(&mut self, current_move: &Move, other: &mut Knot, set: &mut HashSet<(i32, i32)>) {
        let step;
        // suppose this is the coordinate system
        // ^ y
        // |
        // |
        // |
        // |-----------> x
        match current_move.direction {
            Direction::U | Direction::R => step = 1,
            Direction::D | Direction::L => step = -1,
        }
        match current_move.direction {
            Direction::U | Direction::D => {
                for _i in 1..=current_move.amount {
                    self.y += step;
                    other.follow_knot(&self, set);
                }
            }
            Direction::L | Direction::R => {
                for _i in 1..=current_move.amount {
                    self.x += step;
                    other.follow_knot(&self, set);
                }
            }
        }
    }

    fn follow_knot(&mut self, head: &Knot, set: &mut HashSet<(i32, i32)>) {
        if self.x.abs_diff(head.x) <= 1 && self.y.abs_diff(head.y) <= 1 {
            // if head and tail are 1 step away or less, nothing to do
            return;
        } else if self.x == head.x && self.y.abs_diff(head.y) == 2 {
            // if head and tail on same vertical line
            if head.y > self.y {
                self.y += 1;
            } else {
                self.y -= 1;
            }
        } else if self.y == head.y && self.x.abs_diff(head.x) == 2 {
            // if head and tail on same horizontal line
            if head.x > self.x {
                self.x += 1;
            } else {
                self.x -= 1;
            }
        } else {
            // tail needs to move diagonally to catch up to head
            if head.x > self.x {
                self.x += 1;
            } else {
                self.x -= 1;
            }
            if head.y > self.y {
                self.y += 1;
            } else {
                self.y -= 1;
            }
        }

        if !set.contains(&(self.x, self.y)) {
            set.insert((self.x, self.y));
        }
    }
}
// ------------------------ //

// ---------- A2 ---------- //
fn make_move(rope: &mut Vec<Knot>, current_move: &Move, set: &mut HashSet<(i32, i32)>) {
    let step;
    match current_move.direction {
        Direction::U | Direction::R => step = 1,
        Direction::D | Direction::L => step = -1,
    }
    match current_move.direction {
        Direction::U | Direction::D => {
            for _i in 1..=current_move.amount {
                rope[0].y += step;
                follow_knot(rope, set, 1);
            }
        }
        Direction::L | Direction::R => {
            for _i in 1..=current_move.amount {
                rope[0].x += step;
                follow_knot(rope, set, 1);
            }
        }
    }
}

fn follow_knot(rope: &mut Vec<Knot>, set: &mut HashSet<(i32, i32)>, i: usize) {
    if rope[i].x.abs_diff(rope[i - 1].x) <= 1 && rope[i].y.abs_diff(rope[i - 1].y) <= 1 {
        // if head and tail are 1 step away or less, nothing to do
        return;
    } else if rope[i].x == rope[i - 1].x && rope[i].y.abs_diff(rope[i - 1].y) == 2 {
        // if head and tail on same vertical line
        if rope[i - 1].y > rope[i].y {
            rope[i].y += 1;
        } else {
            rope[i].y -= 1;
        }
    } else if rope[i].y == rope[i - 1].y && rope[i].x.abs_diff(rope[i - 1].x) == 2 {
        // if head and tail on same horizontal line
        if rope[i - 1].x > rope[i].x {
            rope[i].x += 1;
        } else {
            rope[i].x -= 1;
        }
    } else {
        // tail needs to move diagonally to catch up to head
        if rope[i - 1].x > rope[i].x {
            rope[i].x += 1;
        } else {
            rope[i].x -= 1;
        }
        if rope[i - 1].y > rope[i].y {
            rope[i].y += 1;
        } else {
            rope[i].y -= 1;
        }
    }

    if i == rope.len() - 1 && !set.contains(&(rope[i].x, rope[i].y)) {
        set.insert((rope[i].x, rope[i].y));
    } else if i < rope.len() - 1 {
        follow_knot(rope, set, i + 1);
    }
}
// ------------------------ //
