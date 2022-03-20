#[macro_use] extern crate tramp;

mod calc;

use clap::Parser;

/// Simple program to calculate the usage of JBL headphones
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Mode of the program.
    /// Modes:
    /// time.
    /// Time - Gets the remaining time from the battery percentage
    #[clap(short, long)]
    mode: String,

    #[clap(short, long, required_if_eq("mode", "time"))]
    percentage: f64,
}

fn main() {
    let args = Args::parse();

    if args.mode == "time".to_owned() {
        println!("{}", calc::time_from_percentage(args.percentage))
    }

    // println!("Time from 1%: {}", calc::time_from_percentage(1.0));
    // println!("Time from 100%: {}", calc::time_from_percentage(100.0));
    // println!("Time from 64%: {}", calc::time_from_percentage(64.0));
    // println!("Time from 23.8%: {}", calc::time_from_percentage(23.8));
    // println!("Time from 30.0%: {}", calc::time_from_percentage(30.0));
}
