use crate::sorter::sort_rules;

mod init;
mod sorter;
#[cfg(test)]
mod tests;

struct Rule {
    low: u8,
    high: u8,
}

fn main() {
    let ordering_rules = init::read_ordering_rules();
    let sorted_rules = sort_rules(ordering_rules);
    let pages = init::read_pages_to_update();

    let mut sum: u64 = 0;
    for page in pages {
        if valid_updates(&sorted_rules, &page) {
            sum += *page.get(page.len().div_ceil(2)).unwrap() as u64;
        }
    }

    println!("{}", sum)
}

fn valid_updates(reference: &Vec<u8>, updates: &Vec<u8>) -> bool {
    let mut ref_i = 0;
    let mut updates_i = 0;

    while ref_i < reference.len() {
        if updates_i == updates.len() {
            return true;
        }

        if updates[updates_i] == reference[ref_i] {
            updates_i += 1;
            ref_i += 1;
        } else {
            ref_i += 1;
        }
    }

    false
}
