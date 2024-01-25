use crate::solution::Solution;
use std::fs;

fn process_calories() -> Vec<i32> {
    let file_path = "data/day01.txt";

    let mut total: i32 = 0;
    let mut totals: Vec<i32> = Vec::new();

    for line in fs::read_to_string(file_path).expect("File should exist").lines() {
        if line == "" {
            totals.push(total);
            total = 0;
        } else {
            total += line.trim().parse::<i32>().unwrap();
        }
    }
    totals.sort();
    totals.reverse();
    totals
}

pub fn part1() -> Solution {
    let totals = process_calories();
    let solution = totals.get(0).unwrap().clone();

    Solution { solution }
}

pub fn part2() -> Solution {
    let elf_totals = process_calories();
    let top_3 = elf_totals.get(0..3).unwrap().iter().sum();

    let solution = top_3;

    Solution { solution }
}