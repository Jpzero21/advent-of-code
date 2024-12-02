use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

// I was extra tired this solution isn't very good :(
fn main() {
    let input_file = File::open("inputs/2023/d03/input.txt").expect("Could not open input file");
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
