/*
    Advent of Code 2022 / Day 2: https://adventofcode.com/2022/day/2
    Q1: What would your total score be if everything goes exactly according to your strategy guide?
    Q2: Following the Elf's instructions for the second column,
        what would your total score be if everything goes exactly according to your strategy guide?
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Score {
    Loss,
    Draw,
    Win,
}

#[derive(Debug)]
enum Shape {
    X,
    Y,
    Z,
    A,
    B,
    C,
}

fn main() {
    let input_file = File::open("./input.txt").expect("Cannot open input file.");
    let file_reader = BufReader::new(&input_file);

    let mut old_total_score = 0;
    let mut new_total_score = 0;
    for l in file_reader.lines() {
        if let Ok(line) = l {
            // ----- A1 ------ //
            let opponent_move = Shape::new(line.chars().nth(0).unwrap()).unwrap();
            let self_move = Shape::new(line.chars().nth(2).unwrap()).unwrap();
            old_total_score += self_move.play(&opponent_move);

            // ----- A2 ----- //
            new_total_score += get_score(&opponent_move, &self_move);
        }
    }

    println!("A1 // Your total score following the first strategy guide is: {old_total_score}");
    println!("A2 // Your total score following the second strategy guide is: {new_total_score}");
}

// --------------- A1 --------------- //

impl Shape {
    fn new(letter: char) -> Result<Shape, Shape> {
        match letter {
            'A' => Ok(Shape::A),
            'B' => Ok(Shape::B),
            'C' => Ok(Shape::C),
            'X' => Ok(Shape::X),
            'Y' => Ok(Shape::Y),
            'Z' => Ok(Shape::Z),
            _ => Err(Shape::A),
        }
    }

    fn play(&self, other: &Shape) -> i32 {
        let mut score: i32 = match self {
            Shape::X => 1,
            Shape::Y => 2,
            Shape::Z => 3,
            _ => 0,
        };
        score += match self.compare(other).unwrap() {
            Score::Loss => 0,
            Score::Draw => 3,
            Score::Win => 6,
        };
        score
    }

    fn compare(&self, other: &Shape) -> Result<Score, Score> {
        match self {
            Shape::X => match other {
                Shape::A => Ok(Score::Draw),
                Shape::B => Ok(Score::Loss),
                Shape::C => Ok(Score::Win),
                _ => Err(Score::Loss),
            },
            Shape::Y => match other {
                Shape::A => Ok(Score::Win),
                Shape::B => Ok(Score::Draw),
                Shape::C => Ok(Score::Loss),
                _ => Err(Score::Loss),
            },
            Shape::Z => match other {
                Shape::A => Ok(Score::Loss),
                Shape::B => Ok(Score::Win),
                Shape::C => Ok(Score::Draw),
                _ => Err(Score::Loss),
            },
            _ => Err(Score::Loss),
        }
    }
}

// --------------- A2 --------------- //

fn get_score(opponent: &Shape, outcome: &Shape) -> i32 {
    match outcome {
        Shape::X => {
            0 + match opponent {
                // to lose to rock, pick scissors = 3
                Shape::A => 3,
                // to lose to paper, pick rock = 1
                Shape::B => 1,
                // to lose to scissors, pick paper = 2
                Shape::C => 2,
                _ => 0,
            }
        }
        Shape::Y => {
            3 + match opponent {
                // pick the same thing as opponent
                Shape::A => 1,
                Shape::B => 2,
                Shape::C => 3,
                _ => 0,
            }
        }
        Shape::Z => {
            6 + match opponent {
                // to win vs rock, pick paper = 2
                Shape::A => 2,
                // to win vs paper, pick scissors = 3
                Shape::B => 3,
                // to win vs scissors, pick rock = 1
                Shape::C => 1,
                _ => 0,
            }
        }
        _ => 0,
    }
}
