fn main() {
    let input = std::fs::read_to_string("inputs/2024/d03/input.txt").unwrap();

    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let multiplications_result = get_muls_result(&re, &input);
    let enabled_multiplications_result = get_muls_result(&re, &filter_out_disabled(input));

    assert_eq!(multiplications_result, 165225049);
    assert_eq!(enabled_multiplications_result, 108830766);

    println!(
        "Adding up all of the results of the multiplications gives {}",
        multiplications_result
    );
    println!(
        "Adding up all of the results of just the enabled multiplications gives {}",
        enabled_multiplications_result
    );
}

fn get_muls_result(re: &regex::Regex, input: &String) -> u64 {
    let mut multiplications_result: u64 = 0;
    for (_, [left, right]) in re.captures_iter(input.as_str()).map(|c| c.extract()) {
        multiplications_result += left.parse::<u64>().unwrap() * right.parse::<u64>().unwrap();
    }
    multiplications_result
}

fn filter_out_disabled(line: String) -> String {
    let do_op = "do()";
    let dont_op = "don't()";
    let mut line = line;

    while let Some(start_cutoff) = line.find(dont_op) {
        let end_cutoff = match &line[start_cutoff + dont_op.len()..].find(do_op) {
            Some(i) => i + start_cutoff + dont_op.len() + do_op.len(),
            None => {
                line = String::from(&line[..start_cutoff]);
                break;
            }
        };

        line = format!("{}{}", &line[..start_cutoff], &line[end_cutoff..]);
    }

    line
}
