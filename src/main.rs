use argh::FromArgs;
mod lib;

#[derive(FromArgs, PartialEq, Debug)]
/// A command with positional arguments.
struct WithPositional {
    #[argh(positional)]
    year: u32,

    /// an optional starting_day which is "0" by default
    #[argh(option, default = "0")]
    starting_day: u32,
}

fn main() {
    let arg: WithPositional = argh::from_env();
    lib::display(arg.year, arg.starting_day)
}
