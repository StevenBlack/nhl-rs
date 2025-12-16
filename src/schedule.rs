use std::u8;

use chrono::DateTime;
use chrono::prelude::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

// constant values
const SCHEDULE_URL: &str = "https://api-web.nhle.com/v1/schedule/now";
const TEAM_SCHEDULE_URL: &str = "https://api-web.nhle.com/v1/club-schedule-season/TTT/20252026";
const PANEL_WIDTH: usize = 55;

// schedule-related data structures
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleRoot {
    // pub next_start_date: String,
    // pub previous_start_date: String,
    pub game_week: Vec<GameWeek>,
    pub pre_season_start_date: String,
    pub regular_season_start_date: String,
    pub regular_season_end_date: String,
    pub playoff_end_date: String,
    pub number_of_games: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamScheduleRoot {
    pub previous_season: i64,
    pub current_season: i64,
    pub club_timezone: String,
    #[serde(rename = "clubUTCOffset")]
    pub club_utcoffset: String,
    pub games: Vec<Game>,
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
    pub game_center_link: Option<String>,
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
pub struct PlaceName {
    pub default: String,
    pub fr: Option<String>,
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

pub fn read_json_from_api() -> ScheduleRoot {
    let response = reqwest::blocking::get(SCHEDULE_URL).unwrap();
    let data = response.text().unwrap();
    let obj: ScheduleRoot = serde_json::from_str(&data).expect("Unable to parse schedule JSON");
    obj
}

pub fn read_team_json_from_api(args: crate::Args) -> TeamScheduleRoot {
    let response = reqwest::blocking::get(TEAM_SCHEDULE_URL.replace("TTT", &args.team.clone().unwrap())).unwrap();
    let data = response.text().unwrap();
    let obj: TeamScheduleRoot = serde_json::from_str(&data).expect("Unable to parse team schedule JSON");
    obj
}

fn get_data() -> ScheduleRoot {
    read_json_from_api()
}

fn get_team_data(args: crate::Args) -> TeamScheduleRoot {

    read_team_json_from_api(args)
}

pub fn schedule() {
    let root = get_data();
    let east_timezone = FixedOffset::west_opt(5 * 3600).unwrap();
    for date in root.game_week {
        schedule_header(date.date.as_str(), date.day_abbrev.as_str());
        for game in date.games {
            if game.game_state == "FUT" {
                print!("{} at {}", game.away_team.abbrev, game.home_team.abbrev);

                let mut dt = DateTime::parse_from_rfc3339(&game.start_time_utc).unwrap();
                dt = dt.with_timezone(&east_timezone);
                print!("  {} ", dt.format("%H:%M").to_string());

                if game.tv_broadcasts.len() > 0 {
                    let mut networks = Vec::new();
                    for broadcast in game.tv_broadcasts {
                        networks.push(broadcast.network);
                    }
                    // print!("  ({})", networks.join(", "));
                    let networks: Vec<String> = networks.into_iter().unique().collect();
                    print!("  ({})", networks.join(", "))
                }
                println!();
                continue;
            }
            print!(
                "{} {} - {} {}",
                game.away_team.abbrev,
                game.away_team.score.unwrap_or(0),
                game.home_team.score.unwrap_or(0),
                game.home_team.abbrev
            );

            let mut dt = DateTime::parse_from_rfc3339(&game.start_time_utc).unwrap();
            dt = dt.with_timezone(&east_timezone);
            print!("  {} ", dt.format("%H:%M").to_string());

            if game.tv_broadcasts.len() > 0 {
                let mut networks = Vec::new();
                for broadcast in game.tv_broadcasts {
                    networks.push(broadcast.network);
                }
                print!("  ({})", networks.join(", "));
            }
            println!();
        }
    }
}

fn schedule_header(title: &str, day: &str) {
    let width = PANEL_WIDTH;
    println!();
    println!("{}", "=".repeat(width));
    let together = format!("{title} ({day})");
    println!("{:^width$}", together);
    println!("{}", "=".repeat(width));
}

// Add ANSI constants and a small helper to color the leading team's abbrev.
const ANSI_BOLD: &str = "\x1b[1m";
const ANSI_RESET: &str = "\x1b[0m";

fn color_abbrev(abbrev: &str, score: i64, opponent_score: i64) -> String {
    if score > opponent_score {
        format!("{}{}{}", ANSI_BOLD, abbrev, ANSI_RESET)
    } else {
        abbrev.to_string()
    }
}

pub fn team_schedule(args: crate::Args) {
    let team = match &args.team {
        Some(t) => t.trim().to_uppercase(),
        None => { eprintln!("No team supplied"); return; }
    };
    let valid_teams = [
        "FLA","TBL","TOR","OTT","MTL","DET","BOS","BUF",
        "WSH","CAR","NJD","CBJ","NYR","NYI","PIT","PHI",
        "WPG","DAL","COL","MIN","STL","CHI","NSH","ARI",
        "VGK","LAK","EDM","CGY","VAN","ANA","SEA","SJS",
        "UTA"
    ];
    if !valid_teams.contains(&team.as_str()) {
        eprintln!("Invalid team code '{}'. Expected one of: {}", team, valid_teams.join(", "));
        return;
    }
    let root = get_team_data(args);
    let east_timezone = FixedOffset::west_opt(5 * 3600).unwrap();
    let mut game_number: u8 = 0;
    for game in root.games {
        if game.game_type == 1 {
            continue; // skip pre-season games
        }
        game_number += 1;
        let mut dt = DateTime::parse_from_rfc3339(&game.start_time_utc).unwrap();
        dt = dt.with_timezone(&east_timezone);
        print!("{:>2} {}  ", game_number,dt.format("%a %b %e %Y").to_string());
        // Future game
        if game.game_state == "FUT" {
            print!("{} at {}", game.away_team.abbrev, game.home_team.abbrev);
            print!("  {} ", dt.format("%H:%M").to_string());
            if game.tv_broadcasts.len() > 0 {
                let mut networks = Vec::new();
                for broadcast in game.tv_broadcasts {
                    networks.push(broadcast.network);
                }
                // print!("  ({})", networks.join(", "));
                let networks: Vec<String> = networks.into_iter().unique().collect();
                print!("  ({})", networks.join(", "))
            }
            println!();
            continue;
        }
        // Pre game
        if game.game_state == "PRE" {
            print!("{} at {}  ", game.away_team.abbrev, game.home_team.abbrev);
            print!("{} ", dt.format("%H:%M").to_string());
            if game.tv_broadcasts.len() > 0 {
                let mut networks = Vec::new();
                for broadcast in game.tv_broadcasts {
                    networks.push(broadcast.network);
                }
                // print!("  ({})", networks.join(", "));
                let networks: Vec<String> = networks.into_iter().unique().collect();
                print!("  ({}) Pre game", networks.join(", "))
            }
            println!();
            continue;
        }
        let game_outcome = match &game.game_outcome {
            Some(outcome) => {
                if outcome.last_period_type == "OT" || outcome.last_period_type == "SO" {
                    format!("({})", outcome.last_period_type)
                } else {
                    "  ".to_string()
                }
            },
            None => {
                eprintln!("No game outcome for completed game");
                continue;
            }
        };
        let away_score = game.away_team.score.unwrap_or(0);
        let home_score = game.home_team.score.unwrap_or(0);
        let away_abbrev = color_abbrev(&game.away_team.abbrev, away_score, home_score);
        let home_abbrev = color_abbrev(&game.home_team.abbrev, home_score, away_score);
        print!(
            "{} {} - {} {} {}",
            away_abbrev,
            away_score,
            home_score,
            home_abbrev,
            game_outcome
        );
        println!();
    }
}
