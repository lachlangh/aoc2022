use crate::solution::Solution;
use std::collections::HashSet;

fn find_duplicate_item_value(rucksack: &str) -> i32 {
    let char_array = rucksack.as_bytes();

    let rucksack_size = char_array.len();

    let compartment_one = &char_array[..(rucksack_size / 2)];
    let compartment_two = &char_array[(rucksack_size / 2)..];

    let mut comp_one_set = HashSet::new();
    let mut comp_two_set = HashSet::new();

    for item in compartment_one.iter() {
        comp_one_set.insert(item);
    }

    for item in compartment_two.iter() {
        comp_two_set.insert(item);
    }

    let mut intersection = comp_one_set.intersection(&comp_two_set);

    let common_object = intersection.next().expect("No common element found");

    utf8_dec_to_score(**common_object)
}

fn utf8_dec_to_score(item: u8) -> i32 {
    if item >= 65 && item <= 90 {
        (item as i32) - 38
    } else if item >= 97 && item <= 122 {
        (item as i32) - 96
    } else {
        panic!("Invalid backpack letter");
    }
}

fn find_common_letter(rucksacks: &Vec<&str>) -> i32 {
    let hash_sacks = rucksacks
        .iter()
        .map(
            |x| make_hash_set(x.as_bytes())
        )
        .collect::<Vec<HashSet<&u8>>>();

    let intersection = hash_sacks
        .get(0).unwrap().intersection(
        hash_sacks.get(1).unwrap()
    );

    let final_sack = hash_sacks.get(2).unwrap();

    for letter in intersection {
        if final_sack.contains(*letter) {
            return utf8_dec_to_score(**letter);
        }
    }

    0 // to satisfy the compiler 
}

fn make_hash_set(char_sack: &[u8]) -> HashSet<&u8> {
    let mut hash_set = HashSet::new();
    for item in char_sack.iter() {
        hash_set.insert(item);
    }
    hash_set
}

pub fn part1() -> Solution {
    let rucksacks = include_str!("../../data/day03.txt");

    let solution: i32 = rucksacks
        .lines()
        .map(find_duplicate_item_value)
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    Solution { solution }
}

pub fn part2() -> Solution {
    let rucksacks = include_str!("../../data/day03.txt").lines();

    let mut score = 0;
    let mut chunk = Vec::with_capacity(3);


    for line in rucksacks {
        chunk.push(line);

        if chunk.len() == 3 {
            score += find_common_letter(&chunk);
            chunk.clear();
        }
    }

    let solution = score;

    Solution { solution }
}