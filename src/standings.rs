use serde::{Deserialize, Serialize};
use std::{fmt};

// constant values
const STANDINGS_URL: &str = "https://api-web.nhle.com/v1/standings/now";
const TEAM_NAME_WIDTH: usize = 15;
const PLACE_NAME_WIDTH: usize = 12;
const GP_WIDTH: usize = 2;
const PLUS_MINUS_WIDTH: usize = 3;
const GOAL_DIFFERENTIAL_WIDTH: usize = 3;
const PANEL_WIDTH: usize = 39;

static CONFERENCES: &[&str] = &["Eastern", "Western"];
static DIVISIONS: &[(&str, &str)] = &[
    ("Eastern", "Atlantic"),
    ("Eastern", "Metropolitan"),
    ("Western", "Central"),
    ("Western", "Pacific"),
];

// standings data structures
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StandingsRoot {
    pub standings: Standings,
}

type Standings = Vec<Standing>;

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
        let gp_width = GP_WIDTH;
        let plus_minus_width = PLUS_MINUS_WIDTH;
        let goal_differential_width = GOAL_DIFFERENTIAL_WIDTH;
        write!(
            f,
            "{} {:>gp_width$} {:>plus_minus_width$} {:>plus_minus_width$}  {:>gp_width$} {:>goal_differential_width$} {:>goal_differential_width$}",
            self.place_name,
            self.games_played,
            self.wins - self.losses,
            self.l10wins - self.l10losses,
            self.regulation_wins,
            self.goal_differential,
            self.l10goal_differential
        )
    }
}

impl fmt::Display for TeamCommonName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let team_name_width = TEAM_NAME_WIDTH;
        write!(f, "{:<team_name_width$}", self.default)
    }
}

impl fmt::Display for PlaceName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let place_name_width = PLACE_NAME_WIDTH;
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

// playoff structures
#[derive(Debug)]
struct Playoffmatchup {
    lbl: String,
    home: Standing,
    away: Standing,
}

type Playoffmatchups = Vec<Playoffmatchup>;

trait CustomDisplay {
    fn custom_display(&self) -> String;
}

impl CustomDisplay for Playoffmatchups {
    fn custom_display(&self) -> String {
        let mut result = String::new();
        for matchup in self {
            let (lbl, home, away) = (&matchup.lbl, &matchup.home, &matchup.away);
            result.push_str(&format!(
                "{} {} ({}) at {} ({})\n",
                lbl,
                away.place_name.default.trim(),
                away.wins - away.losses,
                home.place_name.default.trim(),
                home.wins - home.losses,
            ));
        }
        result
    }
}

struct Cumulator {
    wl: i32,
    rw: i32,
    l10: i32,
    games: i32,
    points: i32,
    goal_diff: i32,
    goal_diff_10: i32,
}

impl Default for Cumulator {
    fn default() -> Self {
        Cumulator {
            wl: 0,
            rw: 0,
            l10: 0,
            games: 0,
            points: 0,
            goal_diff: 0,
            goal_diff_10: 0,
        }
    }
}

impl fmt::Display for Cumulator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:>15}{:4}{:4}{:4} {:3} {:3} {:3} {:.3}",
            "👉🏻",
            self.games,
            self.wl,
            self.l10,
            self.rw,
            self.goal_diff,
            self.goal_diff_10,
            self.points as f64 / ((self.games as f64) * 2.),
        )
    }
}

impl Cumulator {
    fn new() -> Self {
        Cumulator {
            wl: 0,
            l10: 0,
            rw: 0,
            games: 0,
            points: 0,
            goal_diff: 0,
            goal_diff_10: 0,
        }
    }

    fn absorb(&mut self, s: &&Standing) -> () {
        self.wl = self.wl + s.wins - s.losses;
        self.l10 = self.l10 + s.l10wins - s.l10losses;
        self.rw = self.rw + s.regulation_wins;
        self.games = self.games + s.games_played;
        self.points = self.points + s.points;
        self.goal_diff = self.goal_diff + s.goal_differential;
        self.goal_diff_10 = self.goal_diff_10 + s.l10goal_differential;
        ()
    }
}

pub fn read_json_from_api() -> StandingsRoot {
    let response = reqwest::blocking::get(STANDINGS_URL).unwrap();
    let data = response.text().unwrap();
    let obj: StandingsRoot = serde_json::from_str(&data).expect("Unable to parse standings JSON");
    obj
}

fn get_data() -> StandingsRoot {
    read_json_from_api()
}

pub fn standings(args: crate::Args) {
    let mut root = get_data();

    // sort the standings just the way I like it
    root.standings.sort_unstable_by_key(|item| {
        (
            -(item.wins - item.losses),
            item.games_played,
            -item.regulation_wins,
            -item.goal_differential,
        )
    });

    if args.playoffs {
        // display the playoffs picture and bail
        playoff_header();

        let (conf1, conf2): (Vec<Standing>, Vec<Standing>) = root
            .standings
            .into_iter()
            .partition(|s| s.conference_name == CONFERENCES[0]);

        let mut idx = 0;
        for conf in [conf1, conf2] {
            let mut playoffmatchups: Vec<Playoffmatchup> = Vec::new();
            let (div1, div2): (Vec<Standing>, Vec<Standing>) = conf
                .into_iter()
                .partition(|s| s.division_name == DIVISIONS[idx].1);

            let mut firsts: Vec<Standing> = vec![];
            let mut wildcards: Vec<Standing> = vec![];

            for mut div in [div1, div2] {
                // division winners
                firsts.push(div.remove(0));
                // next 2
                playoffmatchups.push(Playoffmatchup {
                    lbl: "[3-2]".to_string(),
                    home: div.remove(0),
                    away: div.remove(0),
                });
                // wildcards bin
                wildcards.extend(div);
            }
            // now sort the firsts
            firsts.sort_unstable_by_key(|item| {
                (
                    -(item.wins - item.losses),
                    item.games_played,
                    -item.regulation_wins,
                )
            });

            // now sort the wildcards again
            wildcards.sort_unstable_by_key(|item| {
                (
                    -(item.wins - item.losses),
                    item.games_played,
                    -item.regulation_wins,
                )
            });

            let lastwc = wildcards[1].wins - wildcards[1].losses;

            // now we can add the wildcards to the playoff matchups
            playoffmatchups.push(Playoffmatchup {
                lbl: "[8-w]".to_string(),
                home: firsts.remove(0),
                away: wildcards.remove(1),
            });

            playoffmatchups.push(Playoffmatchup {
                lbl: "[7-w]".to_string(),
                home: firsts.remove(0),
                away: wildcards.remove(0),
            });

            playoffmatchups.sort_unstable_by_key(|item| {
                (
                    -(item.home.wins - item.home.losses),
                    item.home.games_played,
                    -item.home.regulation_wins,
                )
            });

            println!("{}", playoffmatchups.custom_display());

            // print teams within 3 of the final wildcard
            let mut outsiders = false;
            for wc in wildcards {
                if wc.wins - wc.losses >= lastwc - 3 {
                    if !outsiders {
                        print!("Outside looking-in: ");
                        outsiders = true;
                    }
                    print!(
                        "{} ({}) ",
                        wc.team_abbrev.default.trim(),
                        wc.wins - wc.losses
                    );
                }
            }
            println!();
            println!();

            idx = idx + 2;
        }
        return;
    }

    if args.division {
        bydivision(&root);
    }

    if args.conference {
        byconference(&root);
    }

    if args.full {
        fullleague(&root);
    }

    if args.l10 {
        last10(&root);
    }

    // bail if playoffs or standings were specified
    if args.playoffs || args.division || args.conference || args.full || args.l10 {
        return;
    }

    fn bydivision(r: &StandingsRoot) {
        for division in DIVISIONS {
            let mut cumulator = Cumulator::new();
            standings_header(format!("{} division", division.1).as_str());
            let mut idx = 1;
            for standing in &r.standings {
                if standing.division_name != division.1.to_string() {
                    continue;
                }
                cumulator.absorb(&standing);
                println!("{:>2}. {}", idx, standing);
                idx = idx + 1;
            }
            println!("{}", cumulator);
        }
    }

    fn byconference(r: &StandingsRoot) {
        for conference in CONFERENCES {
            let mut cumulator = Cumulator::new();
            standings_header(format!("{} conference", conference).as_str());
            let mut idx = 1;
            for standing in &r.standings {
                if standing.conference_name != conference.to_string() {
                    continue;
                }
                cumulator.absorb(&standing);
                println!("{:>2}. {}", idx, standing);
                idx = idx + 1;
            }
            println!("{}", cumulator);
        }
    }

    fn fullleague(r: &StandingsRoot) {
        standings_header("Full league");
        let mut cumulator = Cumulator::new();
        let mut idx = 1;
        for standing in &r.standings {
            cumulator.absorb(&standing);
            println!("{:>2}. {}", idx, standing);
            idx = idx + 1;
        }
        println!("{}", cumulator);
    }

    fn last10(r: &StandingsRoot) {
        // sort the standings by the last 10 games
        let mut st = r.standings.clone();
        st.sort_unstable_by_key(|item| {
            (
                -(item.l10wins - item.l10losses),
                -(item.wins - item.losses),
                item.games_played,
                -item.regulation_wins,
            )
        });
        standings_header("Full league (last 10 games)");
        let mut cumulator = Cumulator::new();
        let mut idx = 1;
        for standing in &st {
            cumulator.absorb(&standing);
            println!("{:>2}. {}", idx, standing);
            idx = idx + 1;
        }
        println!("{}", cumulator);
    }

    // iterate our data in various ways
    bydivision(&root);
    byconference(&root);
    fullleague(&root);
    last10(&root);
}

fn standings_header(title: &str) {
    let panel_width = PANEL_WIDTH;
    println!();
    println!("{}", "=".repeat(panel_width));
    println!("{:^panel_width$}", title);
    println!("{}", "=".repeat(panel_width));
    println!(
        "{:>19} {} {}  {}  {} {}",
        "GP", "+/-", "L10", "RW", "GD", "L10"
    );
}

fn playoff_header() {
    let panel_width = 35;
    println!();
    println!("{}", "=".repeat(panel_width));
    println!("{:^panel_width$}", "Playoff Picture");
    println!("{}", "=".repeat(panel_width));
}
