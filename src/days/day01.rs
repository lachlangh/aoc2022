use crate::solution::Solution;
use crate::utils::parse;

fn elf_totals() -> Vec<i32> {
    parse::read_split_paragraphs("data/day01.txt")
        .unwrap()
        .iter()
        .map(|v| v.iter().map(|i| i.parse::<i32>().unwrap()).sum())
        .collect()
}


pub fn part1() -> Solution {
    let totals = elf_totals();
    let solution = *totals.iter().max().unwrap();
    
    Solution { solution }
}

pub fn part2() -> Solution {
    let mut totals = elf_totals();
    totals.sort();
    let solution = totals.iter().rev().take(3).sum();

    Solution { solution }
}