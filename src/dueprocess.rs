use crate::logparse::LogEvent;
use crate::stats::PlayerStats;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Team {
    Attacker,
    Defender,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Biome {
    CStore,
    Dome,
    Factory,
    Killhouse,
}

#[derive(Debug)]
pub struct Kill {
    killer: String,
    killer_team: Option<Team>,
    victim: String,
    victim_team: Team,
}

#[derive(Debug)]
pub struct Round {
    biome: Biome,
    level: String,
    kills: Vec<Kill>,
    winner: Team,
}

#[derive(Debug)]
pub struct Match {
    pub rounds: Vec<Round>
}

impl Match {
    pub fn from_log_events(events: &[LogEvent]) -> Self {
        let mut rounds = Vec::new();
        let mut current_round;

        let mut events_iter = events.into_iter();
        loop {
            let event = events_iter.next();
            if let Some(LogEvent::RoundStart{ level, biome }) = event {
                current_round = Round {
                    winner: Team::Defender,
                    kills: Vec::new(),
                    biome: *biome,
                    level: level.clone(),
                };
                break;
            } else if event == None {
                return Self {
                    rounds
                }
            } 
        }

        for event in events_iter {
            match event {
                LogEvent::MatchStart => break,
                LogEvent::RoundStart{ level, biome } => {
                    rounds.push(current_round);
                    current_round = Round {
                        winner: Team::Defender,
                        kills: Vec::new(),
                        biome: *biome,
                        level: level.clone(),
                    };
                },
                LogEvent::Kill{ killer, victim, killer_team, victim_team } => {
                    current_round.kills.push(Kill {
                        killer: killer.clone(),
                        victim: victim.clone(),
                        killer_team: *killer_team,
                        victim_team: *victim_team,
                    })
                }
                _ => panic!()
            }
        }
        rounds.push(current_round);

        Self { rounds }
    }

    pub fn player_stats(&self) -> Vec<PlayerStats> {
        let mut stats = HashMap::new();

        for round in self.rounds.iter() {
            for kill in round.kills.iter() {
                let victim_stats = stats.entry(kill.victim.clone()).or_insert(PlayerStats {
                    name: kill.victim.clone(), 
                    ..PlayerStats::default()
                });

                if kill.killer_team == None {
                    victim_stats.suicides += 1;
                    continue;
                }

                let killer_insert = PlayerStats {
                    name: kill.killer.clone(),
                    ..PlayerStats::default()
                };

                if kill.killer_team == Some(kill.victim_team) {
                    victim_stats.deaths_to_team += 1;
                    stats.entry(kill.killer.clone()).or_insert(killer_insert).team_kills += 1;
                } else {
                    victim_stats.deaths_to_enemy += 1;
                    stats.entry(kill.killer.clone()).or_insert(killer_insert).enemy_kills += 1;
                }
            }
        }

        stats.drain().map(|(_, v)| v).collect()
    }
}