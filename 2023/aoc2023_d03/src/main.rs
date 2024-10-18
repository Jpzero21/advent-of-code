use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

// I was extra tired this solution isn't very good :(
fn main() {
    let input_file = File::open("./input.txt").expect("Could not open input file");
    let file_reader = BufReader::new(&input_file);

    let mut schematic: Vec<Vec<char>> = vec![];
    for l in file_reader.lines() {
        if let Ok(line) = l {
            let mut current_line = vec![];
            for c in line.chars() {
                current_line.push(c);
            }
            schematic.push(current_line);
        }
    }
    let schematic = schematic;

    let mut total_part_number = 0;
    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for i in 0..schematic.len() {
        let mut j = 0;
        while j < schematic[0].len() {
            if schematic[i][j].is_digit(10) {
                let start_index = j;
                let mut end_index = j;
                while j < schematic[0].len() - 1 && schematic[i][j + 1].is_digit(10) {
                    end_index += 1;
                    j += 1;
                }

                let (adj_to_symbol, gear_locations) =
                    is_adjacent_to_symbol(&schematic, i, start_index, end_index);
                if adj_to_symbol {
                    let mut num_as_str = String::new();
                    for index in start_index..=end_index {
                        num_as_str.push(schematic[i][index]);
                    }
                    let num = num_as_str.parse::<i32>().expect("Could not parse number");
                    total_part_number += num;

                    for gear in gear_locations {
                        gears
                            .entry((gear.0, gear.1))
                            .and_modify(|vec| vec.push(num))
                            .or_insert(vec![num]);
                    }
                }
            }
            j += 1;
        }
    }

    let mut gear_number = 0;
    for (_, numbers) in &gears {
        if numbers.len() == 2 {
            gear_number += numbers[0] * numbers[1];
        }
    }

    println!("The total part number was {}", total_part_number);
    println!("The sum of all of the gear ratios is {}", gear_number);
}

fn is_adjacent_to_symbol(
    schematic: &Vec<Vec<char>>,
    current_row: usize,
    start_index: usize,
    end_index: usize,
) -> (bool, Vec<(usize, usize)>) {
    let row_start = if start_index > 0 {
        start_index - 1
    } else {
        start_index
    };

    let row_end = if end_index < schematic[0].len() - 1 {
        end_index + 1
    } else {
        end_index
    };

    let mut rows_to_check = vec![current_row];
    if current_row > 0 {
        rows_to_check.push(current_row - 1);
    }
    if current_row < schematic.len() - 1 {
        rows_to_check.push(current_row + 1)
    }

    let mut found_a_symbol = false;
    let mut gear_locations = vec![];
    for row in rows_to_check {
        for index in row_start..=row_end {
            if is_symbol(schematic[row][index]) {
                if schematic[row][index] == '*' {
                    gear_locations.push((row, index));
                }
                found_a_symbol = true;
            }
        }
    }

    (found_a_symbol, gear_locations)
}

fn is_symbol(c: char) -> bool {
    !c.is_alphanumeric() && c != '.'
}

/*
--- Day 3: Gear Ratios ---

You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to
the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise.
"Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still
be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can
figure out which one. If you can add up all the part numbers in the engine schematic, it should
be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine.
There are lots of numbers and symbols you don't really understand, but apparently any number
adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum.
(Periods (.) do not count as a symbol.)

Here is an example engine schematic:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..

In this schematic, two numbers are not part numbers because they are not adjacent to a symbol:
114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a
part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers
in the engine schematic?

--- Part Two ---

The engineer finds the missing part and installs it in the engine! As the engine springs to life,
you jump in the closest gondola, finally ready to ascend to the water source.

You don't seem to be going very fast, though. Maybe something is still wrong?
Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.

Before you can explain the situation, she suggests that you look out the window.
There stands the engineer, holding a phone in one hand and waving with the other.
You're going so slowly that you haven't even left the station. You exit the gondola.

The missing part wasn't the only issue - one of the gears in the engine is wrong.
A gear is any * symbol that is adjacent to exactly two part numbers.
Its gear ratio is the result of multiplying those two numbers together.

This time, you need to find the gear ratio of every gear and add them all up so that the engineer
can figure out which gear needs to be replaced.

Consider the same engine schematic again:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..

In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35,
so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490.
(The * adjacent to 617 is not a gear because it is only adjacent to one part number.)
Adding up all of the gear ratios produces 467835.

What is the sum of all of the gear ratios in your engine schematic?

*/
