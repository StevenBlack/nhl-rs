use clap::Parser;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// display the playoffs picture
    #[arg(short, long)]
    playoffs: bool,

    /// display the league proximate schedule
    #[arg(short, long)]
    schedule: bool,

    ///The default team for team schedule is the user's favorite team
    #[arg(short, long)]
    team: Option<String>,

    /// display just division standings
    #[arg(short, long)]
    division: bool,

    /// display just conference standings
    #[arg(short, long)]
    conference: bool,

    /// display just the full league standings
    #[arg(short, long)]
    full: bool,

    /// display full league standings by last 10 games
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
        schedule::schedule(args);
        return;
    }
    standings::standings(args);
}
