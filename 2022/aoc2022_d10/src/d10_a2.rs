use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solution() {
    let file = File::open("./input.txt").expect("Could not open input file.");
    let file_reader = BufReader::new(&file);

    let mut screen = vec![vec![' '; 40]; 6];
    let mut cycle_count: i32 = 0;
    let mut x_register: i32 = 0;

    for l in file_reader.lines() {
        if let Ok(line) = l {
            if line.len() != 0 {
                if line.starts_with("noop") {
                    if should_light_pixel(cycle_count, x_register) {
                        light_pixel(&mut screen, cycle_count);
                    }
                    cycle_count += 1;
                } else {
                    if should_light_pixel(cycle_count, x_register) {
                        light_pixel(&mut screen, cycle_count);
                    }
                    cycle_count += 1;
                    if should_light_pixel(cycle_count, x_register) {
                        light_pixel(&mut screen, cycle_count);
                    }
                    x_register += &line[5..].trim().parse().unwrap();
                    cycle_count += 1;
                }
            }
        }
    }

    println!("------------------ A2 ------------------");
    for vec in screen {
        for c in vec {
            print!("{}", c);
        }
        println!();
    }
}

fn should_light_pixel(cycle_count: i32, x_register: i32) -> bool {
    if ((cycle_count % 40) - 1 - x_register).abs() <= 1 {
        return true;
    }
    false
}

fn light_pixel(screen: &mut Vec<Vec<char>>, cycle_count: i32) {
    let mut row = 0;
    let mut cycle_count = cycle_count;
    while cycle_count - 40 >= 0 {
        row += 1;
        cycle_count -= 40;
    }
    screen[row][cycle_count as usize] = '█';
}
