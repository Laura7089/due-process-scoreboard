use crate::dueprocess::{Team, Biome};
use std::io::{Read, BufRead, BufReader};
use regex::Regex;

lazy_static! {
    static ref KILL_EVENT_REGEX: Regex = Regex::new(r"KillLogUI :: Entry :: <.+><noparse>(?P<killer>.+)</noparse></color> (?P<kill_msg>[A-Z]+) <.+><noparse>(?P<victim>.+)</noparse>").unwrap();
    static ref MATCH_BEGIN_REGEX: Regex = Regex::new(r"StartOfMatchOverlay :: Local Match ID: (?P<matchid>[a-f0-9\-]+)").unwrap();
    static ref ROUND_LOAD_REGEX: Regex = Regex::new(r": Loading Game Level +\[(?P<biome>[a-zA-Z_]+)\] (?P<level>[A-Za-z ]+) \[[-\d]+\]").unwrap();
}

#[derive(Debug, PartialEq)]
pub enum LogEvent {
    MatchStart,
    RoundStart {
        biome: Biome,
        level: String,
    },
    Kill {
        victim: String,
        victim_team: Team,
        killer: String,
        killer_team: Option<Team>,
    },
    RoundEnd {
        winner: Team,
    },
}

impl LogEvent {
    pub fn from_line(line: &str) -> Option<Self> {
        if MATCH_BEGIN_REGEX.is_match(line) {
            Some(Self::MatchStart)
        } else if let Some(captures) = ROUND_LOAD_REGEX.captures(line) {
            Some(Self::RoundStart {
                level: captures["level"].to_string(),
                biome: match &captures["biome"] {
                    "CStore" => Biome::CStore,
                    "Factory" => Biome::Factory,
                    "Killhouse_Day" => Biome::Killhouse,
                    "Dome" => Biome::Dome,
                    other => panic!("Bad biome string: {}", other)
                },
            })
        } else if let Some(captures) = KILL_EVENT_REGEX.captures(line) {
            let killer = captures["killer"].to_string();
            let victim = captures["victim"].to_string();

            Some(match &captures["kill_msg"] {
                "WASTED" => Self::Kill {
                    killer_team: Some(Team::Defender),
                    victim_team: Team::Attacker,
                    killer,
                    victim,
                },
                "NEUTRALIZED" => Self::Kill {
                    killer_team: Some(Team::Attacker),
                    victim_team: Team::Defender,
                    killer,
                    victim,
                },
                "DOUBLE CROSSED" => Self::Kill {
                    killer_team: Some(Team::Defender),
                    victim_team: Team::Defender,
                    killer,
                    victim,
                },
                "DISHONORABLY DISCHARGED" => Self::Kill {
                    killer_team: Some(Team::Attacker),
                    victim_team: Team::Attacker,
                    killer,
                    victim,
                },
                "ROASTED" => Self::Kill {
                    killer_team: None,
                    victim_team: Team::Attacker,
                    killer,
                    victim,
                },
                other => panic!("Bad kill string: {}", other)
            })
        } else {
            None
        }
    }
}

pub fn parse_logs(log_file: impl Read) -> Vec<LogEvent> {
    BufReader::new(log_file)
        .lines()
        .map(|l| LogEvent::from_line(&l.unwrap()))
        .filter(|e| e != &None)
        .map(|e| e.unwrap())
        .collect()
}