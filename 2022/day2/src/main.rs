use core::panic;
use std::fs;

#[derive(Debug)]
struct Round(Hand, Hand);

#[derive(Debug, PartialEq, Eq, Clone)]
enum Hand {
    Rock,
    Paper,
    Scizzors,
}

fn hand_mapping(input: char) -> Hand {
    match input {
        'A' | 'X' => Hand::Rock,
        'B' | 'Y' => Hand::Paper,
        'C' | 'Z' => Hand::Scizzors,
        _ => panic!("Invalid data input!"),
    }
}

fn parse_data(input: &str) -> Vec<Round> {
    let rows: Vec<&str> = input.lines().collect();
    rows.iter()
        .map(|r| {
            Round(
                hand_mapping(r.chars().nth(0).unwrap()),
                hand_mapping(r.chars().nth(2).unwrap()),
            )
        })
        .collect()
}

fn part1(data: &Vec<Round>) -> String {
    use Hand::*;

    let mut points = 0;

    for round in data {
        let Round(_, second) = round;
        match second {
            Rock => points += 1,
            Paper => points += 2,
            Scizzors => points += 3,
        }

        match round {
            Round(first, second) if first == second => points += 3,
            Round(Rock, Paper) | Round(Paper, Scizzors) | Round(Scizzors, Rock) => points += 6,
            _ => (),
        }
    }

    points.to_string()
}

fn part2(data: &Vec<Round>) -> String {
    use Hand::*;
    let mut points = 0;
    let mut my_hand;

    // Kludging this because my model is wrong from part 1. Hang tight.
    for round in data {
        match round {
            Round(their_hand, Rock) => {
                // lose
                my_hand = get_losing_hand(their_hand);
            }
            Round(their_hand, Paper) => {
                // draw
                my_hand = their_hand.clone();
                points += 3;
            }
            Round(their_hand, Scizzors) => {
                // win
                my_hand = get_winning_hand(their_hand);
                points += 6;
            }
        }

        match my_hand {
            Rock => points += 1,
            Paper => points += 2,
            Scizzors => points += 3,
        }
    }

    return points.to_string();

    fn get_losing_hand(hand: &Hand) -> Hand {
        match hand {
            Rock => Scizzors,
            Paper => Rock,
            Scizzors => Paper,
        }
    }

    fn get_winning_hand(hand: &Hand) -> Hand {
        match hand {
            Rock => Paper,
            Paper => Scizzors,
            Scizzors => Rock,
        }
    }
}

fn main() {
    let data = fs::read_to_string("./src/data.txt").unwrap();
    let data = parse_data(&data);

    println!("{}", part1(&data));
    println!("{}", part2(&data));
}

#[cfg(test)]
mod tests {
    use crate::parse_data;
    use crate::part1;
    use crate::part2;
    const EXAMPLE: &str = r#"A Y
B X
C Z"#;

    #[test]
    fn test_part1() {
        let expected = "15".to_string();

        assert_eq!(part1(&parse_data(EXAMPLE)), expected);
    }

    #[test]
    fn test_part2() {
        let expected = "12".to_string();

        assert_eq!(part2(&parse_data(EXAMPLE)), expected);
    }
}
