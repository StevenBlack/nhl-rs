//! A command line tool to display NHL standings and schedules.
use clap::Parser;

/// Structure for storing the command line arguments.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Display the playoffs picture.
    #[arg(short, long)]
    playoffs: bool,

    /// Display the schedule.
    #[arg(short, long)]
    schedule: bool,

    /// Used with --schedule to display just one team's full schedule.
    #[arg(short, long)]
    team: Option<String>,

    /// Display just division standings, starting with divisions in
    /// the Eastern Conference, then the Western Conference.
    #[arg(short, long)]
    division: bool,

    /// Display just conference standings.
    #[arg(short, long)]
    conference: bool,

    /// Display just the full league standings.
    #[arg(short, long)]
    full: bool,

    /// Display full league standings by last 10 games.
    #[arg(long)]
    l10: bool,
}

mod schedule;
mod standings;

fn main() {
    let args = Args::parse();

    if args.schedule {
        if args.team.is_some() {
            schedule::team_schedule(args);
            return;
        }
        schedule::schedule();
        return;
    }
    standings::standings(args);
}
