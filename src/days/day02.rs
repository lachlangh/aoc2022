use std::fs;
use crate::solution::Solution;

#[derive(PartialEq, Eq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

/// Convert character to shape
fn parse_shape(x: char) -> Result<Shape, String> {
    match x {
        'A' | 'X' => Ok(Shape::Rock),
        'B' | 'Y' => Ok(Shape::Paper),
        'C' | 'Z' => Ok(Shape::Scissors),
        _ => Err("Unrecognised shape".to_string())
    }
}

/// Read shapes from puzzle input, odd shapes belong to elf.
fn read_shapes() -> Vec<Shape> {
    return fs::read_to_string("data/day02.txt")
        .unwrap()
        .chars()
        .filter(|s| !s.is_whitespace())
        .map(|s| parse_shape(s).unwrap())
        .collect()
}

/// Return the score for a given shape
fn score_shape(shape: &Shape) -> u8 {
    return match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3
    }
}

/// Return my score depending on whether I win, draw or lose
fn score_match(me: &Shape, elf: &Shape) -> u8 {
    return match (me, elf) {
        (x, y) if x == y => 3,
        (Shape::Rock, Shape::Scissors) => 6,
        (Shape::Paper, Shape::Rock) => 6,
        (Shape::Scissors, Shape::Paper) => 6,
        _ => 0
    }
}

/// Return a new shape based on my and the elf's choices
/// 
/// I will swap a Rock to the losing shape, a Paper to a drawing shape,
/// and a Scissors to a winning shape.
fn swap_shape(me: &Shape, elf: &Shape) -> Shape {
    match me {
        Shape::Rock => {
            match elf {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper
            }
        },
        Shape::Scissors => {
            match elf {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock
            }
        }
        Shape::Paper => elf.clone()
    }
}

pub fn part1() -> Solution {
    let mut moves = read_shapes().into_iter();
    let mut solution: i32 = 0;

    while let Some(elf) = moves.next() {
        if let Some(me) = moves.next() {
            solution += (score_shape(&me) + score_match(&me, &elf)) as i32
        } else {
            break;
        }
    }

    Solution { solution: solution as i32 }
}

pub fn part2() -> Solution {
    let mut moves = read_shapes().into_iter();
    let mut my_shape: Shape;

    let mut solution: i32 = 0;
    while let Some(elf) = moves.next() {
        if let Some(me) = moves.next() {
            my_shape = swap_shape(&me, &elf);
            solution += (score_shape(&my_shape) + score_match(&my_shape, &elf)) as i32
        } else {
            break;
        }
    }

    Solution { solution: solution as i32 }
}