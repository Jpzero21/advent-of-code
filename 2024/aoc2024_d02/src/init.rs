use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_reports() -> Vec<Vec<u32>> {
    let regex = Regex::new(r"(\d+)\s*").unwrap();
    let mut reports: Vec<Vec<u32>> = vec![];

    let file = File::open("inputs/2024/d02/input.txt").expect("Could not open input file");
    let file_reader = BufReader::new(&file);

    for l in file_reader.lines() {
        if l.is_err() {
            panic!("Could not read line from file reader.");
        }
        let line = l.unwrap();
        if line.is_empty() {
            continue;
        }

        reports.push(
            regex
                .find_iter(line.as_str())
                .map(|m| {
                    let level = m.as_str().trim();
                    level
                        .parse::<u32>()
                        .expect(format!("Could not convert '{}' to u32", level).as_str())
                })
                .collect(),
        )
    }

    reports
}
