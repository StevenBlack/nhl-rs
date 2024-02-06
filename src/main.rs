use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub wild_card_indicator: bool,
    pub standings: Vec<Standing>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Standing {
    pub conference_abbrev: String,
    pub conference_home_sequence: i64,
    #[serde(rename = "conferenceL10Sequence")]
    pub conference_l10sequence: i64,
    pub conference_name: String,
    pub conference_road_sequence: i64,
    pub conference_sequence: i64,
    pub date: String,
    pub division_abbrev: String,
    pub division_home_sequence: i64,
    #[serde(rename = "divisionL10Sequence")]
    pub division_l10sequence: i64,
    pub division_name: String,
    pub division_road_sequence: i64,
    pub division_sequence: i64,
    pub game_type_id: i64,
    pub games_played: i64,
    pub goal_differential: i64,
    pub goal_differential_pctg: f64,
    pub goal_against: i64,
    pub goal_for: i64,
    pub goals_for_pctg: f64,
    pub home_games_played: i64,
    pub home_goal_differential: i64,
    pub home_goals_against: i64,
    pub home_goals_for: i64,
    pub home_losses: i64,
    pub home_ot_losses: i64,
    pub home_points: i64,
    pub home_regulation_plus_ot_wins: i64,
    pub home_regulation_wins: i64,
    pub home_ties: i64,
    pub home_wins: i64,
    #[serde(rename = "l10GamesPlayed")]
    pub l10games_played: i64,
    #[serde(rename = "l10GoalDifferential")]
    pub l10goal_differential: i64,
    #[serde(rename = "l10GoalsAgainst")]
    pub l10goals_against: i64,
    #[serde(rename = "l10GoalsFor")]
    pub l10goals_for: i64,
    #[serde(rename = "l10Losses")]
    pub l10losses: i64,
    #[serde(rename = "l10OtLosses")]
    pub l10ot_losses: i64,
    #[serde(rename = "l10Points")]
    pub l10points: i64,
    #[serde(rename = "l10RegulationPlusOtWins")]
    pub l10regulation_plus_ot_wins: i64,
    #[serde(rename = "l10RegulationWins")]
    pub l10regulation_wins: i64,
    #[serde(rename = "l10Ties")]
    pub l10ties: i64,
    #[serde(rename = "l10Wins")]
    pub l10wins: i64,
    pub league_home_sequence: i64,
    #[serde(rename = "leagueL10Sequence")]
    pub league_l10sequence: i64,
    pub league_road_sequence: i64,
    pub league_sequence: i64,
    pub losses: i64,
    pub ot_losses: i64,
    pub place_name: PlaceName,
    pub point_pctg: f64,
    pub points: i64,
    pub regulation_plus_ot_win_pctg: f64,
    pub regulation_plus_ot_wins: i64,
    pub regulation_win_pctg: f64,
    pub regulation_wins: i64,
    pub road_games_played: i64,
    pub road_goal_differential: i64,
    pub road_goals_against: i64,
    pub road_goals_for: i64,
    pub road_losses: i64,
    pub road_ot_losses: i64,
    pub road_points: i64,
    pub road_regulation_plus_ot_wins: i64,
    pub road_regulation_wins: i64,
    pub road_ties: i64,
    pub road_wins: i64,
    pub season_id: i64,
    pub shootout_losses: i64,
    pub shootout_wins: i64,
    pub streak_code: String,
    pub streak_count: i64,
    pub team_name: TeamName,
    pub team_common_name: TeamCommonName,
    pub team_abbrev: TeamAbbrev,
    pub team_logo: String,
    pub ties: i64,
    pub waivers_sequence: i64,
    pub wildcard_sequence: i64,
    pub win_pctg: f64,
    pub wins: i64,
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
fn main() {
    println!("Hello, world!");
}
