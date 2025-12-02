mod day01;
mod day02;

use clap::Parser;

#[derive(Parser, Debug)]
struct PuzzleInput {
    /// Day of the puzzle to run (1-25)
    day: u8,
    /// Part of the puzzle to run (1 or 2)
    part: Part,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Part {
    A,
    B,
    T,
}

fn main() {
    let cli = PuzzleInput::parse();
    match cli.day {
        1 => day01::run(cli.part),
        2 => day02::run(cli.part),
        _ => println!("Day {} is not yet implemented.", cli.day),
    }
}
