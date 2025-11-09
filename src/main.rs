use clap::Parser;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// use local data
    #[arg(short, long)]
    local: bool,

    /// display the playoffs picture
    #[arg(short, long)]
    playoffs: bool,

    /// display the league proximate schedule
    #[arg(short, long)]
    schedule: bool,

    /// display a team's proximate schedule
    #[arg(long)]
    ts: bool,

    ///The default team for team schedule is the user's favorite team
    #[arg(short, long, default_value = "MTL")]
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

    /// Save sample data to file
    #[arg(long)]
    save: bool,
}

mod schedule;
mod standings;
mod team_schedule;

fn main() {
    let args = Args::parse();

    if args.schedule {
        schedule::schedule(args);
        return;
    }
    if args.ts {
        team_schedule::schedule(args);
        return;
    }
    standings::standings(args);
}
