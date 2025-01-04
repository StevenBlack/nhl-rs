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

    /// Save sample data to file
    #[arg(long)]
    save: bool,
}

// constant values
const TEAM_NAME_WIDTH: usize = 15;
const PLACE_NAME_WIDTH: usize = 12;
const GP_WIDTH: usize = 2;
const PLUS_MINUS_WIDTH: usize = 3;
const GOAL_DIFFERENTIAL_WIDTH: usize = 3;
const PANEL_WIDTH: usize = 31;

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
