pub fn count_safe_reports_strict(reports: &Vec<Vec<u32>>) -> i32 {
    let mut number_of_safe_reports = 0;
    reports.iter().for_each(|rep| {
        if utils::is_safe_strict(rep) {
            number_of_safe_reports += 1;
        }
    });

    number_of_safe_reports
}

pub fn count_safe_reports_forgiving(reports: &Vec<Vec<u32>>) -> i32 {
    let mut number_of_safe_reports = 0;
    reports.iter().for_each(|rep| {
        if utils::is_safe_forgiving(rep) {
            number_of_safe_reports += 1;
        }
    });

    number_of_safe_reports
}

mod utils {
    pub fn is_safe_strict(report: &Vec<u32>) -> bool {
        if report.len() <= 1 {
            return true;
        }

        if report[0] == report[1] {
            return false;
        }

        let order = get_report_order(&report);

        for i in 0..report.len() - 1 {
            if !valid_levels(report[i], report[i + 1], &order) {
                return false;
            }
        }

        true
    }

    /*
        The problem with trying to remove an element to fix a report is that you can't be sure if the list
        is ascending or descending until you get to the 4th element.

        From https://www.reddit.com/user/augienaught1/:
            >  [56, 53, 55, 56, 58, 60]
            >  [56, 53, 55, 50, 48, 45]
            > List one is valid if we drop the first element, while list two is valid if we drop the third.
            > The problem has to do with the direction requirement (all increasing or decreasing).

        I found that testing the report without i - 1, i, and i + 1 solves this issue when a report fails.
    */
    pub fn is_safe_forgiving(report: &Vec<u32>) -> bool {
        if report.len() <= 1 {
            return true;
        }

        // helps the get_report_order() to guess the order even if it isn't true
        if report[0] == report[1] {
            let mut remove_curr = report.clone();
            let mut remove_next = report.clone();
            remove_curr.remove(0);
            remove_next.remove(1);
            return is_safe_strict(&remove_curr) || is_safe_strict(&remove_next);
        }

        let order = get_report_order(&report);

        for i in 0..report.len() - 1 {
            let (left, right) = (report[i], report[i + 1]);

            if !valid_levels(left, right, &order) {
                let mut without_prev = report.clone();
                let mut without_curr = report.clone();
                let mut without_next = report.clone();

                without_curr.remove(i);
                without_next.remove(i + 1);
                let mut is_safe = is_safe_strict(&without_curr) || is_safe_strict(&without_next);
                if i > 0 {
                    without_prev.remove(i - 1);
                    is_safe = is_safe || is_safe_strict(&without_prev);
                }

                return is_safe;
            }
        }

        true
    }

    enum Order {
        ASCENDING,
        DESCENDING,
    }

    fn valid_levels(left: u32, right: u32, order: &Order) -> bool {
        let diff = (right as i32) - (left as i32);

        let valid_order = match order {
            Order::ASCENDING => diff > 0,
            Order::DESCENDING => diff < 0,
        };

        let valid_gap = diff.abs() <= 3;

        valid_order && valid_gap
    }

    fn get_report_order(report: &Vec<u32>) -> Order {
        let (left, right) = (report[0], report[1]);

        if left < right {
            Order::ASCENDING
        } else if left > right {
            Order::DESCENDING
        } else {
            panic!("Something went wrong getting the order");
        }
    }
}
