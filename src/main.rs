// CLI for Advent of Code solutions.
// 
// To run day `d` simply write `cargo run -- d` at the prompt. 
// For example, `cargo run -- 5`. You can also choose to run a
// single part of the chosen day, e.g. `cargo run -- 5 2` will
// run the solution for day 5, part 2.

use std::env;
use std::process;
use aoc2022::config::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|e| {
        eprintln!("Failed to parse arguments: {e}");
        process::exit(1);
    });

    if let Err(e) = aoc2022::run(config) {
        eprintln!("Execution failed: {e}");
        process::exit(1);
    }
}
