/*
    Advent of Code 2022 / Day 11: https://adventofcode.com/2022/day/11
    --- Day 11: Monkey in the Middle ---
    Q1: What is the level of monkey business after 20 rounds of stuff-slinging simian shenanigans?
    Q2: Starting again from the initial state in your puzzle input, what is the level of monkey
        business after 10000 rounds?
*/

struct Monkey {
    id: usize,
    items: Vec<u128>,
    num_of_inspections: u128,
}

fn main() {
    // --------------- A1 --------------- //
    let mut monkeys: Vec<Monkey> = Vec::new();
    init_vector(&mut monkeys);

    for _ in 0..20 {
        for mi in 0..monkeys.len() {
            monkeys[mi].num_of_inspections += monkeys[mi].items.len() as u128;
            let modifications = play_round(&monkeys[mi], true);
            monkeys[mi].items.clear();
            for (destination, item) in modifications {
                monkeys[destination].items.push(item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.num_of_inspections.cmp(&a.num_of_inspections));
    println!(
        "A1 // The level of monkey business after 20 rounds of \
        stuff-slinging simian shenanigans is {}",
        monkeys[0].num_of_inspections * monkeys[1].num_of_inspections
    );

    // --------------- A2 --------------- //
    monkeys.clear();
    init_vector(&mut monkeys);

    for _ in 0..10000 {
        for mi in 0..monkeys.len() {
            monkeys[mi].num_of_inspections += monkeys[mi].items.len() as u128;
            let modifications = play_round(&monkeys[mi], false);
            monkeys[mi].items.clear();
            for (destination, item) in modifications {
                monkeys[destination].items.push(item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.num_of_inspections.cmp(&a.num_of_inspections));
    println!(
        "A2 // The level of monkey business after 10000 rounds is {}",
        monkeys[0].num_of_inspections * monkeys[1].num_of_inspections
    );
}

fn play_round(monkey: &Monkey, should_divide: bool) -> Vec<(usize, u128)> {
    let mut modifications = Vec::new();
    for item in monkey.items.clone() {
        let mut worry = monkey.get_level_of_worry(item);
        if should_divide {
            worry /= 3;
        } else {
            // to manage the worry levels from overflowing we need to keep them contained
            // since all the tests are can divide by X, you can keep it within sane ranges
            // by making sure it doesn't surpass the product of all X values
            // i.e. worry %= X-Product, which would not mess up the test cases
            worry %= 9699690;
        }
        let throw_recipient = monkey.get_throw_recipient(worry);
        modifications.push((throw_recipient, worry));
    }
    modifications
}

impl Monkey {
    fn new(_id: usize, _items: Vec<u128>) -> Monkey {
        Monkey {
            id: _id,
            items: _items,
            num_of_inspections: 0,
        }
    }

    fn get_level_of_worry(&self, old: u128) -> u128 {
        (match self.id {
            0 => old * 11,
            1 => old * 17,
            2 => old + 8,
            3 => old + 3,
            4 => old + 4,
            5 => old + 7,
            6 => old * old,
            7 => old + 6,
            _ => 0,
        }) as u128
    }

    fn get_throw_recipient(&self, worry: u128) -> usize {
        match self.id {
            0 => return if worry % 7 == 0 { 6 } else { 7 },
            1 => return if worry % 13 == 0 { 5 } else { 2 },
            2 => return if worry % 5 == 0 { 4 } else { 5 },
            3 => return if worry % 19 == 0 { 6 } else { 0 },
            4 => return if worry % 2 == 0 { 0 } else { 3 },
            5 => return if worry % 11 == 0 { 3 } else { 4 },
            6 => return if worry % 17 == 0 { 1 } else { 7 },
            7 => return if worry % 3 == 0 { 2 } else { 1 },
            _ => 0,
        }
    }
}

fn init_vector(monkeys: &mut Vec<Monkey>) {
    monkeys.push(Monkey::new(0, vec![66, 79]));
    monkeys.push(Monkey::new(1, vec![84, 94, 94, 81, 98, 75]));
    monkeys.push(Monkey::new(2, vec![85, 79, 59, 64, 79, 95, 67]));
    monkeys.push(Monkey::new(3, vec![70]));
    monkeys.push(Monkey::new(4, vec![57, 69, 78, 78]));
    monkeys.push(Monkey::new(5, vec![65, 92, 60, 74, 72]));
    monkeys.push(Monkey::new(6, vec![77, 91, 91]));
    monkeys.push(Monkey::new(7, vec![76, 58, 57, 55, 67, 77, 54, 99]));
}
