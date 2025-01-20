use clap::Parser;
#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// use local data
    #[arg(short, long)]
    local: bool,

    /// display the playoffs picture
    #[arg(short, long)]
    playoffs: bool,

    /// display the proximate schedule
    #[arg(short, long)]
    schedule: bool,

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

fn main() {
    let args = Args::parse();

    if args.schedule {
        schedule::schedule(args);
        return;
    }
    standings::standings(args);
}
