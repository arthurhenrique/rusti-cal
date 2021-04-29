use argh::FromArgs;
mod lib;

#[derive(FromArgs, PartialEq, Debug)]
/// A command with positional arguments.
struct WithPositional {
    #[argh(positional)]
    year: u32,
}

fn main() {
    let arg: WithPositional = argh::from_env();
    lib::display(arg.year, lib::calendar(arg.year))
}
