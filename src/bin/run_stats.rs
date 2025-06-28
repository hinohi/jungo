use jungo::stats::run_statistics;

fn main() {
    println!("=== Jungo Statistics Runner ===\n");

    // Run statistics for 5x5 board
    let stats_5x5 = run_statistics(5, 10000);
    stats_5x5.print_summary(10000, 5);

    println!("\n");

    // Run statistics for 7x7 board
    let stats_7x7 = run_statistics(7, 10000);
    stats_7x7.print_summary(10000, 7);
}
