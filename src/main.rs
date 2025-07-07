use argh::FromArgs;
use locale_config::Locale;

use rusti_cal::{display, current_date};

#[derive(FromArgs, PartialEq, Debug)]
/// A command with positional arguments.
struct WithPositional {
    #[argh(positional, default = "default_year()")]
    year: u32,

    /// an optional starting_day which is "0" by default
    #[argh(option, default = "0")]
    starting_day: u32,

    /// an optional flag for enabling colored output
    #[argh(switch, short = 'c')]
    color: bool,

    /// an optional flag for enabling week numbers
    #[argh(switch, short = 'w')]
    week_numbers: bool,
}

fn default_year() -> u32 {
    let (year, _m, _d) = current_date();
    year
}

fn locale() -> String {
    let locale = Locale::user_default();
    locale
        .tags()
        .next()
        .map(|(_, x)| x.to_string().replace("-", "_"))
        .unwrap_or_default()
}

fn main() {
    let arg = argh::from_env::<WithPositional>();
    display(
        arg.year,
        &locale(),
        arg.starting_day,
        !arg.color,
        arg.week_numbers,
    );
}
