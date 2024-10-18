use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let base_10 = 10;

    // --- Part One ---
    let input_file = File::open("./input.txt").expect("Cannot open input file");
    let file_reader = BufReader::new(&input_file);
    let mut total = 0;
    for l in file_reader.lines() {
        if let Ok(line) = l {
            if line.is_empty() {
                continue;
            }

            let mut first_digit: Option<u32> = None;
            let mut second_digit: u32 = 0;
            for c in line.chars() {
                if !c.is_digit(base_10) {
                    continue;
                }
                if first_digit.is_none() {
                    first_digit = c.to_digit(base_10);
                }
                second_digit = c.to_digit(base_10).expect("Could not parse to int");
            }
            let string_value = format!(
                "{}{}",
                first_digit.ok_or(0).unwrap().to_string(),
                second_digit.to_string()
            );
            total += string_value.parse::<i32>().unwrap();
        }
    }
    println!("The first total was {}", total);

    // --- Part Two ---
    let input_file = File::open("./input.txt").expect("Cannot open input file");
    let file_reader = BufReader::new(&input_file);
    let mut actual_total = 0;
    for l in file_reader.lines() {
        if let Ok(line) = l {
            if line.is_empty() {
                continue;
            }

            let mut first_digit: Option<u32> = None;
            let mut second_digit: u32 = 0;

            for mut i in 0..line.len() {
                if let Some(c) = line.chars().nth(i) {
                    if c.is_digit(base_10) {
                        if first_digit.is_none() {
                            first_digit = c.to_digit(base_10);
                        }
                        second_digit = c.to_digit(base_10).expect("Could not parse to int");
                    } else {
                        if let (Some(c), skip) = starts_with_alpha_digit(&line[i..]) {
                            if first_digit.is_none() {
                                first_digit = Some(c);
                            }
                            second_digit = c;
                            i += skip;
                        }
                    }
                }
            }

            let string_value = format!(
                "{}{}",
                first_digit
                    .ok_or(0)
                    .expect("First digit was None")
                    .to_string(),
                second_digit.to_string()
            );
            actual_total += string_value.parse::<i32>().unwrap();
        }
    }

    println!("The actual total was {}", actual_total);
}

fn starts_with_alpha_digit(str: &str) -> (Option<u32>, usize) {
    return if str.starts_with("one") {
        (Some(1), "one".len())
    } else if str.starts_with("two") {
        (Some(2), "two".len())
    } else if str.starts_with("three") {
        (Some(3), "three".len())
    } else if str.starts_with("four") {
        (Some(4), "four".len())
    } else if str.starts_with("five") {
        (Some(5), "five".len())
    } else if str.starts_with("six") {
        (Some(6), "six".len())
    } else if str.starts_with("seven") {
        (Some(7), "seven".len())
    } else if str.starts_with("eight") {
        (Some(8), "eight".len())
    } else if str.starts_with("nine") {
        (Some(9), "nine".len())
    } else {
        (None, 0)
    };
}

/*
--- Day 1: Trebuchet?! ---

Something is wrong with global snow production, and you've been selected to take a look.
The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that
are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all
fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in
the Advent calendar; the second puzzle is unlocked when you complete the first.
Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") and where they're
even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions")
and hang on did you just say the sky ("of course, where do you think snow comes from") when you
realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

As they're making the final adjustments, they discover that their calibration document (your puzzle input)
has been amended by a very young Elf who was apparently just excited to show off her art skills.
Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally contained
a specific calibration value that the Elves now need to recover. On each line, the calibration value
can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?

--- Part Two ---

Your calculation isn't quite right.
It looks like some of the digits are actually spelled out with letters:
one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen

In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

What is the sum of all of the calibration values?

*/
