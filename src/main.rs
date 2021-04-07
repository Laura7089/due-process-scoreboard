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

    let matches: Vec<_> = log_events
        .split(|e| e == &LogEvent::MatchStart)
        .map(dueprocess::Match::from_log_events)
        .filter(|m| m.rounds.len() != 0)
        .collect();

    for dpmatch in matches {
        let mut match_table = Table::new();
        match_table.add_row(stats::PlayerStats::table_header());

        for stats in dpmatch.player_stats().iter() {
            match_table.add_row(stats.to_table_row());
        }

        match_table.printstd();
    }
}
