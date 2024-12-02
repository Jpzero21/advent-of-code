mod init;
mod solution;

fn main() {
    let reports = init::get_reports();
    println!(
        "{} reports are initially safe.",
        solution::count_safe_reports_strict(&reports)
    );
    println!(
        "{} reports are safe after using the problem dampener.",
        solution::count_safe_reports_forgiving(&reports)
    );

    assert_eq!(solution::count_safe_reports_strict(&reports), 572);
    assert_eq!(solution::count_safe_reports_forgiving(&reports), 612);
}
