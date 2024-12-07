mod sorter;

struct Rule {
    low: u8,
    high: u8,
}

fn main() {
    let all_rules: Vec<Rule> = vec![
        Rule { low: 47, high: 53 },
        Rule { low: 47, high: 53 },
        Rule { low: 97, high: 13 },
        Rule { low: 97, high: 61 },
        Rule { low: 97, high: 47 },
        Rule { low: 75, high: 29 },
        Rule { low: 61, high: 13 },
        Rule { low: 75, high: 53 },
        Rule { low: 29, high: 13 },
        Rule { low: 97, high: 29 },
        Rule { low: 53, high: 29 },
        Rule { low: 61, high: 53 },
        Rule { low: 97, high: 53 },
        Rule { low: 61, high: 29 },
        Rule { low: 47, high: 13 },
        Rule { low: 75, high: 47 },
        Rule { low: 97, high: 75 },
        Rule { low: 47, high: 61 },
        Rule { low: 75, high: 61 },
        Rule { low: 47, high: 29 },
        Rule { low: 75, high: 13 },
        Rule { low: 53, high: 13 },
    ];

    let sorted_rules = sorter::sort_rules(all_rules);
    println!("sorted rules: {:?}", sorted_rules);
    assert_eq!(sorted_rules, vec![97, 75, 47, 61, 53, 29, 13]);
}
