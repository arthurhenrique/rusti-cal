use std::env;
mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    let year = args[1].parse::<u32>().unwrap();
    lib::display(year, lib::calendar(year))
}
