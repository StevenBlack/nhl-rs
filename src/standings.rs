use std::{fmt, fs};
use serde::{Deserialize, Serialize};

// constant values
const STANDINGS_URL: &str = "https://api-web.nhle.com/v1/standings/now";
const STANDINGS_FILE: &str = "./sample_standings.json";

// standings data structures
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StandingsRoot {
    pub standings: Vec<Standing>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Standing {
    pub conference_abbrev: String,
    pub conference_home_sequence: i32,
    #[serde(rename = "conferenceL10Sequence")]
    pub conference_l10sequence: i32,
    pub conference_name: String,
    pub conference_road_sequence: i32,
    pub conference_sequence: i32,
    pub date: String,
    pub division_abbrev: String,
    pub division_home_sequence: i32,
    #[serde(rename = "divisionL10Sequence")]
    pub division_l10sequence: i32,
    pub division_name: String,
    pub division_road_sequence: i32,
    pub division_sequence: i32,
    pub game_type_id: i32,
    pub games_played: i32,
    pub goal_differential: i32,
    pub goal_differential_pctg: f64,
    pub goal_against: i32,
    pub goal_for: i32,
    pub goals_for_pctg: f64,
    pub home_games_played: i32,
    pub home_goal_differential: i32,
    pub home_goals_against: i32,
    pub home_goals_for: i32,
    pub home_losses: i32,
    pub home_ot_losses: i32,
    pub home_points: i32,
    pub home_regulation_plus_ot_wins: i32,
    pub home_regulation_wins: i32,
    pub home_ties: i32,
    pub home_wins: i32,
    #[serde(rename = "l10GamesPlayed")]
    pub l10games_played: i32,
    #[serde(rename = "l10GoalDifferential")]
    pub l10goal_differential: i32,
    #[serde(rename = "l10GoalsAgainst")]
    pub l10goals_against: i32,
    #[serde(rename = "l10GoalsFor")]
    pub l10goals_for: i32,
    #[serde(rename = "l10Losses")]
    pub l10losses: i32,
    #[serde(rename = "l10OtLosses")]
    pub l10ot_losses: i32,
    #[serde(rename = "l10Points")]
    pub l10points: i32,
    #[serde(rename = "l10RegulationPlusOtWins")]
    pub l10regulation_plus_ot_wins: i32,
    #[serde(rename = "l10RegulationWins")]
    pub l10regulation_wins: i32,
    #[serde(rename = "l10Ties")]
    pub l10ties: i32,
    #[serde(rename = "l10Wins")]
    pub l10wins: i32,
    pub league_home_sequence: i32,
    #[serde(rename = "leagueL10Sequence")]
    pub league_l10sequence: i32,
    pub league_road_sequence: i32,
    pub league_sequence: i32,
    pub losses: i32,
    pub ot_losses: i32,
    pub place_name: PlaceName,
    pub point_pctg: f64,
    pub points: i32,
    pub regulation_plus_ot_win_pctg: f64,
    pub regulation_plus_ot_wins: i32,
    pub regulation_win_pctg: f64,
    pub regulation_wins: i32,
    pub road_games_played: i32,
    pub road_goal_differential: i32,
    pub road_goals_against: i32,
    pub road_goals_for: i32,
    pub road_losses: i32,
    pub road_ot_losses: i32,
    pub road_points: i32,
    pub road_regulation_plus_ot_wins: i32,
    pub road_regulation_wins: i32,
    pub road_ties: i32,
    pub road_wins: i32,
    pub season_id: i32,
    pub shootout_losses: i32,
    pub shootout_wins: i32,
    pub streak_code: String,
    pub streak_count: i32,
    pub team_name: TeamName,
    pub team_common_name: TeamCommonName,
    pub team_abbrev: TeamAbbrev,
    pub team_logo: String,
    pub ties: i32,
    pub waivers_sequence: i32,
    pub wildcard_sequence: i32,
    pub win_pctg: f64,
    pub wins: i32,
}

impl fmt::Display for Standing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        let gp_width = crate::GP_WIDTH;
        let plus_minus_width = crate::PLUS_MINUS_WIDTH;
        write!(f, "{} {:>gp_width$} {:>plus_minus_width$} {:>plus_minus_width$}",
        self.place_name,
        self.games_played,
        self.wins - self.losses,
        self.l10wins - self.l10losses,
    )
    }
}

impl fmt::Display for TeamCommonName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let team_name_width = crate::TEAM_NAME_WIDTH;
        write!(f, "{:<team_name_width$}", self.default)
    }
}

impl fmt::Display for PlaceName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let place_name_width = crate::PLACE_NAME_WIDTH;
        write!(f, "{:<place_name_width$}", self.default)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceName {
    pub default: String,
    pub fr: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamName {
    pub default: String,
    pub fr: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamCommonName {
    pub default: String,
    pub fr: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamAbbrev {
    pub default: String,
}

pub fn read_json_from_file () -> StandingsRoot {
    let path = STANDINGS_FILE;
    let data = fs::read_to_string(path).expect("Unable to read standings JSON file");
    let obj: StandingsRoot = serde_json::from_str(&data).expect("Unable to parse standings JSON");
    obj
}

pub fn read_json_from_api () -> StandingsRoot {
    let response = reqwest::blocking::get(STANDINGS_URL).unwrap();
    let data = response.text().unwrap();
    let obj: StandingsRoot = serde_json::from_str(&data).expect("Unable to parse standings JSON");
    obj
}

fn get_data() -> StandingsRoot {
    if crate::LOCAL_DATA {
        read_json_from_file()
    } else {
        read_json_from_api()
    }
}

pub fn standings(_args: crate::Args) {
    // standings;
    // some basic groupings
    let conferences = vec!["Eastern", "Western"];
    let divisions = vec![
        ("Eastern", "Atlantic"),
        ("Eastern", "Metropolitan"),
        ("Western", "Central"),
        ("Western", "Pacific"),
    ];
    let mut root = get_data();
    let mut idx;

    // sort the standings just the way I like it
    root.standings.sort_unstable_by_key(|item| (
        -(item.wins - item.losses),
        item.games_played,
        -item.regulation_wins
    ));

    // iterate our data in various ways
    for division in &divisions {
        standings_header(format!("{} division", division.1).as_str());
        idx = 1;
        for standing in &root.standings {
            if standing.division_name != division.1.to_string() {
                continue;
            }
            println!("{:>2}. {}", idx, standing);
            idx = idx + 1;
        }
    }

    for conference in &conferences {
        standings_header(format!("{} conference", conference).as_str());
        idx = 1;
        for standing in &root.standings {
            if standing.conference_name != conference.to_string() {
                continue;
            }
            println!("{:>2}. {}", idx, standing);
            idx = idx + 1;
        }
    }

    standings_header("Full league");
    idx = 1;
    for standing in &root.standings {
        println!("{:>2}. {}", idx, standing);
        idx = idx + 1;
    }
}

fn standings_header(title: &str) {
    let panel_width = crate::PANEL_WIDTH;
    println!();
    println!("{}", "=".repeat(panel_width));
    println!("{:^panel_width$}", title);
    println!("{}", "=".repeat(panel_width));
    println!("{:>19} {} {}", "GP", "+/-", "L10");
}