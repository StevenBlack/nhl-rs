use std::{fmt, fs};
use serde::{Deserialize, Serialize};
use clap::Parser;


#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    schedule: bool,
}

// constant values
const STANDINGS_URL: &str = "https://api-web.nhle.com/v1/standings/now";
const SCHEDULE_URL: &str = "https://api-web.nhle.com/v1/schedule/now";
const LOCAL_DATA: bool = false;
const TEAM_NAME_WIDTH: usize = 15;
const PLACE_NAME_WIDTH: usize = 12;
const GP_WIDTH: usize = 2;
const PLUS_MINUS_WIDTH: usize = 3;
const PANEL_WIDTH: usize = 27;

// data structures
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
        write!(f, "{} {:>GP_WIDTH$} {:>PLUS_MINUS_WIDTH$} {:>PLUS_MINUS_WIDTH$}",
        self.place_name,
        self.games_played,
        self.wins - self.losses,
        self.l10wins - self.l10losses,
    )
    }
}

impl fmt::Display for TeamCommonName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:<TEAM_NAME_WIDTH$}", self.default)
    }
}

impl fmt::Display for PlaceName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:<PLACE_NAME_WIDTH$}", self.default)
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

// schedule-related data structures

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleRoot {
    pub next_start_date: String,
    pub previous_start_date: String,
    pub game_week: Vec<GameWeek>,
    pub odds_partners: Vec<OddsPartner>,
    pub pre_season_start_date: String,
    pub regular_season_start_date: String,
    pub regular_season_end_date: String,
    pub playoff_end_date: String,
    pub number_of_games: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameWeek {
    pub date: String,
    pub day_abbrev: String,
    pub number_of_games: i64,
    pub games: Vec<Game>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: i64,
    pub season: i64,
    pub game_type: i64,
    pub venue: Venue,
    pub neutral_site: bool,
    #[serde(rename = "startTimeUTC")]
    pub start_time_utc: String,
    #[serde(rename = "easternUTCOffset")]
    pub eastern_utcoffset: String,
    #[serde(rename = "venueUTCOffset")]
    pub venue_utcoffset: String,
    pub venue_timezone: String,
    pub game_state: String,
    pub game_schedule_state: String,
    pub tv_broadcasts: Vec<TvBroadcast>,
    pub away_team: AwayTeam,
    pub home_team: HomeTeam,
    pub period_descriptor: PeriodDescriptor,
    pub game_outcome: Option<GameOutcome>,
    pub winning_goalie: Option<WinningGoalie>,
    pub winning_goal_scorer: Option<WinningGoalScorer>,
    pub three_min_recap: Option<String>,
    pub three_min_recap_fr: Option<String>,
    pub game_center_link: String,
    pub tickets_link: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Venue {
    pub default: String,
    pub fr: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TvBroadcast {
    pub id: i64,
    pub market: String,
    pub country_code: String,
    pub network: String,
    pub sequence_number: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AwayTeam {
    pub id: i64,
    pub place_name: PlaceName,
    pub abbrev: String,
    pub logo: String,
    pub dark_logo: String,
    pub away_split_squad: bool,
    pub score: Option<i64>,
    pub radio_link: Option<String>,
    pub odds: Option<Vec<Odd>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Odd {
    pub provider_id: i64,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HomeTeam {
    pub id: i64,
    pub place_name: PlaceName2,
    pub abbrev: String,
    pub logo: String,
    pub dark_logo: String,
    pub home_split_squad: bool,
    pub score: Option<i64>,
    pub radio_link: Option<String>,
    pub odds: Option<Vec<Odd2>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceName2 {
    pub default: String,
    pub fr: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Odd2 {
    pub provider_id: i64,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeriodDescriptor {
    pub number: Option<i64>,
    pub period_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameOutcome {
    pub last_period_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WinningGoalie {
    pub player_id: i64,
    pub first_initial: FirstInitial,
    pub last_name: LastName,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstInitial {
    pub default: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastName {
    pub default: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WinningGoalScorer {
    pub player_id: i64,
    pub first_initial: FirstInitial2,
    pub last_name: LastName2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstInitial2 {
    pub default: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastName2 {
    pub default: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OddsPartner {
    pub partner_id: i64,
    pub country: String,
    pub name: String,
    pub image_url: String,
    pub site_url: Option<String>,
    pub bg_color: String,
    pub text_color: String,
    pub accent_color: String,
}



pub fn read_standings_json_from_file () -> StandingsRoot {
    let path = "./sample.json";
    let data = fs::read_to_string(path).expect("Unable to read standings JSON file");
    let obj: StandingsRoot = serde_json::from_str(&data).expect("Unable to parse standings JSON");
    obj
}

pub fn read_standings_json_from_api () -> StandingsRoot {
    let response = reqwest::blocking::get(STANDINGS_URL).unwrap();
    let data = response.text().unwrap();
    let obj: StandingsRoot = serde_json::from_str(&data).expect("Unable to parse standings JSON");
    obj
}

pub fn read_schedule_json_from_api () -> ScheduleRoot {
    let response = reqwest::blocking::get(SCHEDULE_URL).unwrap();
    let data = response.text().unwrap();
    let obj: ScheduleRoot = serde_json::from_str(&data).expect("Unable to parse schedule JSON");
    obj
}

fn get_standings_data() -> StandingsRoot {
    if LOCAL_DATA {
        read_standings_json_from_file()
    } else {
        read_standings_json_from_api()
    }
}

fn standings() {
    // standings;
    // some basic groupings
    let conferences = vec!["Eastern", "Western"];
    let divisions = vec![
        ("Eastern", "Atlantic"),
        ("Eastern", "Metropolitan"),
        ("Western", "Central"),
        ("Western", "Pacific"),
    ];
    let mut root = get_standings_data();
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

fn schedule() {
    let response = reqwest::blocking::get(SCHEDULE_URL).unwrap();
    let data = response.text().unwrap();
    let obj: ScheduleRoot = serde_json::from_str(&data).expect("Unable to parse schedule JSON");
    for date in obj.game_week {
        schedule_header( date.date.as_str(), date.day_abbrev.as_str());
        for game in date.games {
            if game.game_state == "FUT" {
                println!(
                    "{} at {}",
                    game.away_team.place_name.default,
                    game.home_team.place_name.default
                );
                continue;
            }
            println!("{} {} - {} {}",
                game.away_team.place_name.default,
                game.away_team.score.unwrap(),
                game.home_team.score.unwrap(),
                game.home_team.place_name.default
            );
        }
    }
}

fn main() {
    let args = Args::parse();

    if args.schedule {
        schedule();
        return;
    }
    standings();
}

fn standings_header(title: &str) {
    println!();
    println!("{}", "=".repeat(PANEL_WIDTH));
    println!("{:^PANEL_WIDTH$}", title);
    println!("{}", "=".repeat(PANEL_WIDTH));
    println!("{:>19} {} {}", "GP", "+/-", "L10");
}

fn schedule_header(title: &str, day: &str) {
    println!();
    println!("{}", "=".repeat(PANEL_WIDTH));
    let together = format!("{title} ({day})");
    println!("{:^PANEL_WIDTH$}", together);
    println!("{}", "=".repeat(PANEL_WIDTH));
}