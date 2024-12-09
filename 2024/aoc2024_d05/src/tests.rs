use super::*;

#[test]
fn it_works() {
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
    let valid = vec![75, 47, 61, 53, 29];
    let invalid = vec![75, 97, 47, 61, 53];

    assert_eq!(sorted_rules, vec![97, 75, 47, 61, 53, 29, 13]);
    assert_eq!(valid_updates(&sorted_rules, &valid), true);
    assert_eq!(valid_updates(&sorted_rules, &invalid), false);
}
