use argh::FromArgs;
use chrono::{Datelike, Local};
use locale_config::Locale;

use rusti_cal::display;

#[derive(FromArgs, PartialEq, Debug)]
/// Display a localized calendar for a year.
struct Args {
    #[argh(positional, default = "default_year()", from_str_fn(parse_year))]
    year: u32,

    /// first weekday, from Sunday (0) through Saturday (6)
    #[argh(option, default = "0", from_str_fn(parse_starting_day))]
    starting_day: u32,

    /// an optional flag for enabling colored output
    #[argh(switch, short = 'c')]
    color: bool,

    /// an optional flag for enabling week numbers
    #[argh(switch, short = 'w')]
    week_numbers: bool,
}

fn default_year() -> u32 {
    let now = Local::now();
    let (_, year) = now.year_ce();
    year
}

fn locale() -> String {
    let locale = Locale::user_default();
    locale
        .tags()
        .next()
        .map(|(_, x)| x.to_string().replace('-', "_"))
        .unwrap_or_default()
}

fn parse_year(value: &str) -> Result<u32, String> {
    value
        .parse()
        .ok()
        .filter(|year| (1..=9999).contains(year))
        .ok_or_else(|| "year must be between 1 and 9999".to_owned())
}

fn parse_starting_day(value: &str) -> Result<u32, String> {
    value
        .parse()
        .ok()
        .filter(|day| *day < 7)
        .ok_or_else(|| "starting day must be between 0 (Sunday) and 6 (Saturday)".to_owned())
}

fn main() {
    let arg = argh::from_env::<Args>();
    display(
        arg.year,
        &locale(),
        arg.starting_day,
        !arg.color,
        arg.week_numbers,
    );
}
