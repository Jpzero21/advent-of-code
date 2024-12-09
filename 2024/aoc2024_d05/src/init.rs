use crate::Rule;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_ordering_rules() -> Vec<Rule> {
    let input = std::fs::read_to_string("inputs/2024/d05/ordering_rules.txt").unwrap();
    let re = regex::Regex::new(r"(\d+)\|(\d+)").unwrap();
    let mut all_rules = vec![];
    for (_, [low, high]) in re.captures_iter(input.as_str()).map(|c| c.extract()) {
        all_rules.push(Rule {
            low: low.parse::<u8>().unwrap(),
            high: high.parse::<u8>().unwrap(),
        })
    }

    all_rules
}

pub fn read_pages_to_update() -> Vec<Vec<u8>> {
    let file_reader = match File::open("inputs/2024/d05/pages_to_update.txt") {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("Could not open input file pages_to_update: {}", e),
    };
    let re = regex::Regex::new(r"(\d+)").unwrap();

    let mut pages: Vec<Vec<u8>> = vec![];

    for l in file_reader.lines() {
        if let Ok(line) = l {
            let mut page: Vec<u8> = vec![];
            for cap in re.captures_iter(line.as_str()) {
                page.push(cap[0].parse::<u8>().unwrap());
            }
            pages.push(page)
        }
    }

    pages
}
