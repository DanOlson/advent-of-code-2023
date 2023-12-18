use std::collections::{HashSet, HashMap};
use regex::Regex;

pub struct Card {
    pub id: usize,
    pub winning_numbers: HashSet<usize>,
    pub my_numbers: HashSet<usize>,
}

impl Card {
    pub fn points(&self) -> usize {
        let match_count = self.match_count();
        if match_count == 0 {
            0
        } else {
            let mut points: usize = 1;
            for _m in 0..(match_count - 1) {
                points *= 2;
            }
            points
        }
    }

    pub fn match_count(&self) -> usize {
        let intersection = self.winning_numbers
            .intersection(&self.my_numbers)
            .collect::<HashSet<&usize>>();
        intersection.len()
    }
}

impl TryFrom<&str> for Card {
    type Error = &'static str;

    fn try_from(candidate: &str) -> Result<Self, Self::Error> {
        let reg = Regex::new(r"Card\s+(?<id>\d+):\s(?<winning_nums>[\d+|\s+]+)\|(?<my_nums>[\d+|\s+]+)$").unwrap();
        if let Some(caps) = reg.captures(candidate) {
            let winning_numbers = caps["winning_nums"]
                .split_ascii_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<HashSet<usize>>();
            let my_numbers = caps["my_nums"]
                .split_ascii_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<HashSet<usize>>();
            Ok(Card {
                id: caps["id"].parse().unwrap(),
                winning_numbers,
                my_numbers
            })
        } else {
            Err("failed to parse str into Card")
        }
    }
}

pub fn add(input: Vec<&str>) -> usize {
    input
        .iter()
        .filter_map(|line| {
            if let Ok(card) = Card::try_from(*line) {
                Some(card.points())
            } else {
                None
            }
        })
        .sum()
}

pub fn count_copies(input: Vec<&str>) -> usize {
    let mut counts_by_card_id: HashMap<usize, usize> = HashMap::new();
    input
        .iter()
        .filter_map(|line| {
            if let Ok(card) = Card::try_from(*line) { Some(card) } else { None }
        })
        .for_each(|card| {
            let count = counts_by_card_id
                .entry(card.id)
                .or_insert(0);
            *count += 1;
            (0..*count).for_each(|_i| {
                (1..=card.match_count()).for_each(|offset| {
                    let id = card.id + offset;
                    *counts_by_card_id
                        .entry(id)
                        .or_insert(0) += 1;
                })
            })
        });
    counts_by_card_id.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_card_copies_with_sample() {
        let input = include_str!("../input/sample.txt")
            .split_terminator('\n')
            .collect::<Vec<&str>>();
        let result = count_copies(input);
        assert_eq!(result, 30);
    }

    #[test]
    fn count_card_copies_with_input() {
        let input = include_str!("../input/input.txt")
            .split_terminator('\n')
            .collect::<Vec<&str>>();
        let result = count_copies(input);
        assert_eq!(result, 5704953);
    }

    #[test]
    fn sum_card_points_with_sample() {
        let input = include_str!("../input/sample.txt")
            .split_terminator('\n')
            .collect::<Vec<&str>>();
        let result = add(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn sum_card_points_with_input() {
        let input = include_str!("../input/input.txt")
            .split_terminator('\n')
            .collect::<Vec<&str>>();
        let result = add(input);
        assert_eq!(result, 19135);
    }

    #[test]
    fn test_card_from_str_is_ok() {
        let candidate = "Card     1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let result = Card::try_from(candidate);
        assert!(result.is_ok());
        let card = result.unwrap();
        assert_eq!(card.id, 1);
        assert_eq!(card.winning_numbers, HashSet::from([41, 48, 83, 86, 17]));
        assert_eq!(card.my_numbers, HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]));
    }

    #[test]
    fn test_card_points() {
        let card = Card {
            id: 1,
            winning_numbers: HashSet::from([41, 48, 83, 86, 17]),
            my_numbers: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53])
        };
        assert_eq!(card.points(), 8);

        let card_no_points = Card {
            id: 1,
            winning_numbers: HashSet::from([21, 49, 82, 96, 27]),
            my_numbers: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53])
        };
        assert_eq!(card_no_points.points(), 0);

        let card_one_point = Card {
            id: 1,
            winning_numbers: HashSet::from([21, 49, 83, 96, 27]),
            my_numbers: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53])
        };
        assert_eq!(card_one_point.points(), 1);
    }

    #[test]
    fn test_card_from_str_is_err() {
        let result = Card::try_from("asdf");
        assert!(result.is_err());
    }
}
