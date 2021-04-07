#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prettytable;

mod logparse;
mod dueprocess;
mod stats;

use logparse::LogEvent;
use std::path::PathBuf;
use prettytable::Table;

fn main() {

    let log_path: PathBuf = std::env::args().nth(1).expect("No log file passed").into();
    // let log_reader = BufReader::new(File::open(log_path).unwrap());
    let log_content = std::fs::read_to_string(log_path).unwrap();

    // for match_str in MATCH_BEGIN_REGEX.split(&log_content).skip(1) {
    //     println!("{:?}", MatchInProgress::from_log_segment(match_str));
    // }

    let log_events: Vec<LogEvent> = log_content
        .lines()
        .map(LogEvent::from_line)
        .filter(|e| e != &None)
        .map(|e| e.unwrap())
        .collect();

    let first_match = dueprocess::Match::from_log_events(&log_events);

    let mut first_match_table = Table::new();
    first_match_table.add_row(stats::PlayerStats::table_header());

    for stats in first_match.player_stats().iter() {
        first_match_table.add_row(stats.table_row());
    }

    first_match_table.printstd();
}
