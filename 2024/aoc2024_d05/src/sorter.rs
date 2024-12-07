use crate::Rule;
use std::collections::HashMap;

struct Memory {
    greater_than: Option<u8>,
    lower_than: Option<u8>,
}

pub fn sort_rules(all_rules: Vec<Rule>) -> Vec<u8> {
    let mut order: Vec<u8> = vec![];
    let mut memories: HashMap<u8, Memory> = HashMap::new();
    let mut all_rules = all_rules;

    while !all_rules.is_empty() {
        all_rules = all_rules
            .into_iter()
            .filter(|r| {
                // If sorted array is empty, directly insert both values and delete rule
                if order.is_empty() {
                    order.push(r.low);
                    order.push(r.high);
                    return false;
                }

                // Find how many bounds are known for this current rule
                let low_i = get_index(&order, &r.low);
                let high_i = get_index(&order, &r.high);

                // Both bounds are known
                // (#, #)
                if low_i.is_some() && high_i.is_some() {
                    // The rule is useless
                    return false;
                }

                // Neither bound is known
                // (?, ?)
                if low_i.is_none() && high_i.is_none() {
                    // you should keep the rule for a future iteration
                    return true;
                }

                if let Some(index) = low_i {
                    // We have the lower edge
                    // -----------------
                    //  @index in arr[]
                    //  |
                    // (#, ?)
                    // -----------------
                    known_lower_bound(&mut order, &mut memories, r, index);
                } else if let Some(index) = high_i {
                    // We have the upper edge
                    // -----------------
                    //    @index in arr[]
                    //     |
                    // (?, #)
                    // -----------------
                    known_upper_bound(&mut order, &mut memories, r, index);
                }

                false
            })
            .collect();
    }

    order
}

fn known_lower_bound(
    order: &mut Vec<u8>,
    memories: &mut HashMap<u8, Memory>,
    rule: &Rule,
    lower_bound_index: usize,
) {
    let new_number = rule.high;

    // If there is nothing after the lower bound, we can directly insert
    if lower_bound_index.eq(&(order.len() - 1)) {
        order.push(new_number);
        return;
    }

    // Check what is stored in memory
    match memories.get_mut(&new_number) {
        Some(mem) => {
            // There is already some knowledge about this number
            // Do we have previous information on this number's lower bounds?
            match mem.greater_than {
                Some(x) => {
                    // Can we make the information more precise? In this case,
                    // can we bring the lower bound closer to our new number?
                    if let Some(old_gt_index) = get_index(&order, &x) {
                        if lower_bound_index > old_gt_index {
                            // We can narrow down our range
                            mem.greater_than = Some(rule.low);
                        } else {
                            // We did not gain any new information
                            return;
                        }
                    }
                }
                None => {
                    mem.greater_than = Some(rule.low);
                }
            };

            if let Some(lower_than) = mem.lower_than {
                if let Some(lt_index) = get_index(order, &lower_than) {
                    attempt_insert(order, lower_bound_index, lt_index, new_number);
                }
            }
        }
        None => {
            memories.insert(
                new_number,
                Memory {
                    greater_than: None,
                    lower_than: Some(rule.high),
                },
            );
        }
    }
}

fn known_upper_bound(
    order: &mut Vec<u8>,
    memories: &mut HashMap<u8, Memory>,
    rule: &Rule,
    upper_bound_index: usize,
) {
    let new_number = rule.low;

    // If there is nothing below the upper bound, we can directly insert
    if upper_bound_index.eq(&0) {
        order.insert(0, new_number);
        return;
    }

    // Check what is stored in memory
    match memories.get_mut(&new_number) {
        Some(mem) => {
            // There is already some knowledge about this number

            // Do we have information on this number's upper bounds?
            match mem.lower_than {
                Some(x) => {
                    // Can we make the information more precise? In this case,
                    // can we bring the upper bound closer to our new number?
                    if let Some(old_lt_index) = get_index(&order, &x) {
                        if upper_bound_index < old_lt_index {
                            // We can narrow down our range
                            mem.lower_than = Some(rule.high);
                        } else {
                            // We did not gain any new information
                            return;
                        }
                    }
                }
                None => {
                    mem.lower_than = Some(rule.high);
                }
            };

            if let Some(greater_than) = mem.greater_than {
                if let Some(gt_index) = get_index(order, &greater_than) {
                    attempt_insert(order, gt_index, upper_bound_index, new_number);
                }
            }
        }
        None => {
            memories.insert(
                new_number,
                Memory {
                    greater_than: None,
                    lower_than: Some(rule.high),
                },
            );
        }
    }
}

fn attempt_insert(
    order: &mut Vec<u8>,
    greater_than_index: usize,
    lower_than_index: usize,
    new_number: u8,
) {
    if lower_than_index == greater_than_index + 1 {
        // our sorted array looks like
        //
        //       greater_than_index
        //              |             lower_than_index
        //              |               |
        // [x, x, x, greater_than, lower_than, x, x, x]
        //                        |
        //                  insert new page number here
        //
        order.insert(lower_than_index, new_number);
    }
}

fn get_index(arr: &Vec<u8>, x: &u8) -> Option<usize> {
    arr.iter().position(|&el| el.eq(&x))
}
