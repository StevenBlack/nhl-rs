use clap::Parser;
#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    schedule: bool,
}

// constant values
const LOCAL_DATA: bool = false;
const TEAM_NAME_WIDTH: usize = 15;
const PLACE_NAME_WIDTH: usize = 12;
const GP_WIDTH: usize = 2;
const PLUS_MINUS_WIDTH: usize = 3;
const PANEL_WIDTH: usize = 27;

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
