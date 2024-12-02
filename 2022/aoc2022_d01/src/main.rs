use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_file = File::open("inputs/2022/d01/input.txt").expect("Cannot open input file.");
    let file_reader = BufReader::new(&input_file);

    let mut all_elves: Vec<i32> = Vec::new();
    let mut current_elf: i32 = 0;

    for l in file_reader.lines() {
        if let Ok(line) = l {
            if line.len() != 0 {
                // keep adding to current elf
                let line: i32 = line.trim().parse().expect("Error parsing line.");
                current_elf += line;
            } else {
                // new line, current elf ends here
                all_elves.push(current_elf);
                current_elf = 0;
            }
        }
    }
    all_elves.push(current_elf);
    all_elves.sort_by(|a, b| b.cmp(a));

    println!("The elf with most calories has {} calories", all_elves[0]);
    println!(
        "The top three elves have {} calories when combined",
        all_elves[0] + all_elves[1] + all_elves[2]
    );
}
