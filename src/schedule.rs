use itertools::Itertools;
use std::fs;
use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use chrono::DateTime;

// constant values
const SCHEDULE_URL: &str = "https://api-web.nhle.com/v1/schedule/now";
const SCHEDULE_FILE: &str = "./sample_schedule.json";
const PANEL_WIDTH: usize = 55;
// schedule-related data structures

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleRoot {
    pub next_start_date: String,
    pub previous_start_date: String,
    pub game_week: Vec<GameWeek>,
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


pub fn read_json_from_file(_args: crate::Args) -> ScheduleRoot {
    let path = SCHEDULE_FILE;
    let data = fs::read_to_string(path).expect("Unable to read schedule JSON file");
    let obj: ScheduleRoot = serde_json::from_str(&data).expect("Unable to parse schedule JSON");
    obj
}

pub fn read_json_from_api(args: crate::Args) -> ScheduleRoot {
    let response = reqwest::blocking::get(SCHEDULE_URL).unwrap();
    let data = response.text().unwrap();
    let obj: ScheduleRoot = serde_json::from_str(&data).expect("Unable to parse schedule JSON");
    if args.save {
        println!("Writing to file.");
        fs::write(SCHEDULE_FILE, data).expect("Unable to write schedule JSON file");
    }
    obj
}

fn get_data(args: crate::Args) -> ScheduleRoot {
    if args.local {
        read_json_from_file(args)
    } else {
        read_json_from_api(args)
    }
}

pub fn schedule(args: crate::Args) {
    let root = get_data(args);
    let east_timezone = FixedOffset::west_opt(5 * 3600).unwrap();
    for date in root.game_week {
        schedule_header( date.date.as_str(), date.day_abbrev.as_str());
        for game in date.games {
            if game.game_state == "FUT" {
                print!(
                    "{} at {}",
                    game.away_team.place_name.default,
                    game.home_team.place_name.default
                );

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
            print!("{} {} - {} {}",
                game.away_team.place_name.default,
                game.away_team.score.unwrap_or(0),
                game.home_team.score.unwrap_or(0),
                game.home_team.place_name.default
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