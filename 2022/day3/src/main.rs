#![feature(iter_array_chunks)]
use std::{collections::HashMap, fs};

fn part1(data: &str) -> usize {
    let mut sum_of_priorities: usize = 0;
    let item_positions: HashMap<char, usize> = {
        let items = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        items.char_indices().map(|(i, c)| (c, i + 1)).collect()
    };
    let rucksacks = data.lines();
    for sack in rucksacks {
        let compartment1 = sack[..sack.len() / 2].to_string();
        let compartment2 = sack[sack.len() / 2..].to_string();
        // println!("{} {}", compartment1.len(), compartment2.len());

        for item in compartment1.chars() {
            if let Some(_c) = compartment2.find(item) {
                sum_of_priorities += item_positions.get(&item).unwrap();
                break;
            }
        }
    }
    sum_of_priorities
}

fn part2(data: &str) -> usize {
    let mut sum_of_priorities: usize = 0;
    let item_positions: HashMap<char, usize> = {
        let items = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        items.char_indices().map(|(i, c)| (c, i + 1)).collect()
    };
    let rucksacks = data.lines();

    // Probably should just make a hashmap counting chars. Instead I'm going to try nest
    // loops as deeply as possible, to curse this solution.

    // for [s1, s2, s3] in rucksacks.array_chunks() {
    for [s1, s2, s3] in rucksacks.array_chunks() {
        // This is neat!
        'outer: for c in s1.chars() {
            for c2 in s2.chars() {
                for c3 in s3.chars() {
                    if c == c2 && c2 == c3 {
                        sum_of_priorities += item_positions.get(&c).unwrap();
                        break 'outer; // yay loop labels lol
                    }
                }
            }
        }
    }
    sum_of_priorities
}

fn main() {
    let data = fs::read_to_string("./src/data.txt").unwrap();

    println!("{}", part1(&data));
    println!("{}", part2(&data));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_test() {
        assert_eq!(part1(EXAMPLE), 157);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(EXAMPLE), 70);
    }
}
