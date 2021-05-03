use argh::FromArgs;
use chrono::prelude::*;

mod lib;

#[derive(FromArgs, PartialEq, Debug)]
/// A command with positional arguments.
struct WithPositional {
    #[argh(positional)]
    year: Option<u32>,

    /// an optional starting_day which is "0" by default
    #[argh(option, default = "0")]
    starting_day: u32,
}

fn default_year() -> u32 {
    let now = Local::now();
    let (_, year) = now.year_ce();
    year
}

fn main() {
    let arg: WithPositional = argh::from_env();
    let year = match arg.year {
        Some(y) => y,
        None => default_year(),
    };
    lib::display(year, lib::calendar(year, arg.starting_day))
}
