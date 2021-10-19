use argh::FromArgs;
use chrono::prelude::*;
use locale_config::Locale;

use rusti_cal::display;

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

fn locale() -> String {
    let locale = Locale::user_default();
    let mut tag = locale.tags();
    match tag.next() {
        Some((_, x)) => x.to_string().replace("-", "_"),
        None => "".to_string(),
    }
}

fn main() {
    let arg = argh::from_env::<WithPositional>();
    let year = match arg.year {
        Some(y) => y,
        None => default_year(),
    };
    display(year, &locale(), arg.starting_day);
}
