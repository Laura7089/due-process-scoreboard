#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prettytable;

mod logparse;
mod dueprocess;
mod stats;

use logparse::LogEvent;
use prettytable::Table;
use std::path::PathBuf;
use std::fs::File;

fn main() {
    let log_path: PathBuf = std::env::args().nth(1).expect("No log file passed").into();
    let log_events = logparse::parse_logs(File::open(log_path).unwrap());

    let first_match = dueprocess::Match::from_log_events(&log_events);

    let mut first_match_table = Table::new();
    first_match_table.add_row(stats::PlayerStats::table_header());

    for stats in first_match.player_stats().iter() {
        first_match_table.add_row(stats.table_row());
    }

    first_match_table.printstd();
}
