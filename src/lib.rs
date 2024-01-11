pub mod config;
mod days;
mod solution;

use std::error::Error;
use std::time::Instant;
use solution::Solution;

use crate::config::Config;
use crate::days::{
    day01, day02, day03, day04, day05,
    day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15,
    day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
};
    
// Run the code specified by the CLI inputs
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let clock = Instant::now();

    match config.part {
        Some(p) => {
            println!("\nRunning day {:02}, part {}...", config.day, p);
            let soln = run_part(config.day, p);
            println!("  - {:?}", soln);
        },
        None => {
            println!("\nRunning day {:02}...", config.day);
            let soln1 = run_part(config.day, 1);
            let soln2 = run_part(config.day, 2);
            println!("  - Part one: {:?}", soln1);
            println!("  - Part two: {:?}", soln2);
        }
    }

    let elapsed = clock.elapsed().as_millis();
    println!("  - Elapsed: {:.2}ms", elapsed);

    Ok(())
}

fn run_part(day: u8, part: u8) -> Solution{
    if part == 1 {
        match day {
            1 =>  day01::part1(),
            2 =>  day02::part1(),
            3 =>  day03::part1(),
            4 =>  day04::part1(),
            5 =>  day05::part1(),
            6 =>  day06::part1(),
            7 =>  day07::part1(),
            8 =>  day08::part1(),
            9 =>  day09::part1(),
            10 => day10::part1(),
            11 => day11::part1(),
            12 => day12::part1(),
            13 => day13::part1(),
            14 => day14::part1(),
            15 => day15::part1(),
            16 => day16::part1(),
            17 => day17::part1(),
            18 => day18::part1(),
            19 => day19::part1(),
            20 => day20::part1(),
            21 => day21::part1(),
            22 => day22::part1(),
            23 => day23::part1(),
            24 => day24::part1(),
            25 => day25::part1(),
            _  => unimplemented!(),
        }
    } else if part == 2 {
        match day {
            1 =>  day01::part2(),
            2 =>  day02::part2(),
            3 =>  day03::part2(),
            4 =>  day04::part2(),
            5 =>  day05::part2(),
            6 =>  day06::part2(),
            7 =>  day07::part2(),
            8 =>  day08::part2(),
            9 =>  day09::part2(),
            10 => day10::part2(),
            11 => day11::part2(),
            12 => day12::part2(),
            13 => day13::part2(),
            14 => day14::part2(),
            15 => day15::part2(),
            16 => day16::part2(),
            17 => day17::part2(),
            18 => day18::part2(),
            19 => day19::part2(),
            20 => day20::part2(),
            21 => day21::part2(),
            22 => day22::part2(),
            23 => day23::part2(),
            24 => day24::part2(),
            25 => day25::part2(),
            _  => unimplemented!(),
        }
    } else {
        panic!("part {} undefined", part)
    }
}
