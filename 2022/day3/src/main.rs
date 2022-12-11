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
            if let Some(c) = compartment2.find(item) {
                sum_of_priorities += item_positions.get(&item).unwrap();
                break;
            }
        }
    }
    sum_of_priorities
}

fn main() {
    let data = fs::read_to_string("./src/data.txt").unwrap();

    println!("{}", part1(&data));
}

#[cfg(test)]
mod tests {
    use crate::part1;

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
}
