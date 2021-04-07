use prettytable::{Cell, Row, format::Alignment};

#[derive(Debug, Default)]
pub struct PlayerStats {
    pub name: String,
    pub enemy_kills: u8,
    pub team_kills: u8,
    pub deaths_to_enemy: u8,
    pub deaths_to_team: u8,
    pub suicides: u8,
}

impl PlayerStats {
    pub fn table_header() -> Row {
        row!["Player Name", "Kills", "Team Kills", "Deaths"]
    }

    pub fn to_table_row(&self) -> Row {
        let cells = vec![
            Cell::new_align(&self.name, Alignment::LEFT),
            Cell::new_align(&format!("{}", self.enemy_kills), Alignment::RIGHT),
            Cell::new_align(&format!("{}", self.team_kills), Alignment::RIGHT),
            Cell::new_align(&format!("{}", self.deaths_to_enemy + self.deaths_to_team + self.suicides), Alignment::RIGHT),
        ];

        Row::new(cells)
    }
}