mod day01;
mod day02;
mod day03;
mod day04;

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
        3 => day03::run(cli.part),
        4 => day04::run(cli.part),
        _ => println!("Day {} is not yet implemented.", cli.day),
    }
}
