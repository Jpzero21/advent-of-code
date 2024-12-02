use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input_file = File::open("inputs/2023/d02/input.txt").expect("Could not open input file");
    let file_reader = BufReader::new(&input_file);

    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    let mut id_total = 0;
    let mut power_sum = 0;
    for l in file_reader.lines() {
        if let Ok(line) = l {
            let max_red = get_color_value(&line[..], &"red");
            let max_green = get_color_value(&line[..], &"green");
            let max_blue = get_color_value(&line[..], &"blue");

            if max_red <= red_limit && max_green <= green_limit && max_blue <= blue_limit {
                id_total += get_line_id(&line[..]);
            }

            power_sum += max_red * max_blue * max_green;
        }
    }

    println!("The total of the game IDs is {}", id_total);
    println!("The sum of the power of these sets is {}", power_sum);
}

fn get_color_value(line: &str, color: &str) -> i32 {
    let re = Regex::new(format!(r#"(\d{}) {}"#, "{1,}", color).as_str()).unwrap();

    let mut max = 0;
    for (_, [value]) in re.captures_iter(line).map(|c| c.extract()) {
        let current = value.parse::<i32>().unwrap();
        if current > max {
            max = current;
        }
    }

    max
}

fn get_line_id(line: &str) -> i32 {
    let re = Regex::new(r"Game (?<id>\d{1,})").unwrap();

    let Some(caps) = re.captures(line) else {
        panic!("Game didn't have an ID");
    };

    caps["id"].parse::<i32>().expect("Could not parse game ID")
}
