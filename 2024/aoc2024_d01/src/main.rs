fn main() {
    let (first_list, second_list) = init::read_lists();

    println!(
        "PART 1: The total distance between the lists is {}",
        part_1::get_total_distance(&first_list, &second_list)
    );

    println!(
        "PART 2: The similarity score between the two lists is {}",
        part_2::get_similarity_score(&first_list, &second_list)
    )
}

mod init {
    use regex::Regex;
    use std::fs::File;
    use std::io::Read;

    pub fn read_lists() -> (Vec<u32>, Vec<u32>) {
        let re = match Regex::new(r"(\d+)\s+(\d+)") {
            Ok(re) => re,
            Err(e) => panic!("Could not init regex: {}", e),
        };

        let mut first_list = vec![];
        let mut second_list = vec![];

        let buffer = get_input_as_string();
        for (_, [first, second]) in re.captures_iter(buffer.as_str()).map(|c| c.extract()) {
            first_list.push(first.parse::<u32>().unwrap());
            second_list.push(second.parse::<u32>().unwrap());
        }

        first_list.sort();
        second_list.sort();

        (first_list, second_list)
    }

    fn get_input_as_string() -> String {
        let mut buffer = String::new();

        match File::open("inputs/2024/d01/input.txt") {
            Ok(mut file) => {
                file.read_to_string(&mut buffer)
                    .expect("Could not convert input file to string");
            }
            Err(error) => panic!("Could not open input.txt: {:?}", error),
        }

        buffer
    }
}

mod part_1 {
    pub fn get_total_distance(first_list: &Vec<u32>, second_list: &Vec<u32>) -> u32 {
        if first_list.len() != second_list.len() {
            panic!("Could not get total distance - lists are of different lengths");
        }

        let mut total_distance: u32 = 0;
        for i in 0..first_list.len() {
            total_distance += first_list[i].abs_diff(second_list[i])
        }

        total_distance
    }
}

mod part_2 {
    use std::collections::HashMap;

    pub fn get_similarity_score(first_list: &Vec<u32>, second_list: &Vec<u32>) -> u32 {
        let mut score: u32 = 0;

        let location_id_frequencies = get_location_id_frequencies(second_list);

        for id in first_list {
            score += match location_id_frequencies.get(id) {
                Some(value) => id * value,
                None => 0,
            }
        }

        score
    }

    fn get_location_id_frequencies(second_list: &Vec<u32>) -> HashMap<u32, u32> {
        let mut frequencies: HashMap<u32, u32> = HashMap::new();

        for id in second_list {
            frequencies.insert(
                *id,
                match frequencies.get(id) {
                    Some(&value) => value + 1,
                    None => 1,
                },
            );
        }

        frequencies
    }
}
