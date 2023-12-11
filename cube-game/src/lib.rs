use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    num::ParseIntError
};
use regex::Regex;

pub struct Config {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

pub struct Game {
    pub id: u32,
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

pub enum Rgb {
    RED,
    GREEN,
    BLUE
}

fn parse_round(round: &str) -> Vec<(Rgb, u32)> {
    let mut outcome = vec![];
    let reg = Regex::new(r"(\d+)\s(red|green|blue)").unwrap();
    for (_, [count, color]) in reg.captures_iter(round).map(|c| c.extract()) {
        let c = match color {
            "red" => Rgb::RED,
            "green" => Rgb::GREEN,
            "blue" => Rgb::BLUE,
            _ => Rgb::BLUE
        };
        outcome.push((c, count.parse().unwrap()))
    }
    outcome
}

impl TryFrom<&String> for Game {
    type Error = ParseIntError;

    fn try_from(line: &String) -> Result<Self, Self::Error> {
        let reg = Regex::new(r"^Game (\d+): (.*)$").unwrap();
        let mut id: u32 = 0;
        let mut red_count: u32 = 0;
        let mut green_count: u32 = 0;
        let mut blue_count: u32 = 0;
        if !reg.is_match(line) {
            println!("Could not match {line}");
            "".parse::<u32>()?;
        }
        for (_, [game_id, rest]) in reg.captures_iter(line).map(|cap| cap.extract()) {
            id = game_id.parse()?;
            rest.split(';')
                .map(|round| parse_round(round.trim()))
                .for_each(|scores| {
                    scores.iter().for_each(|(color, score)| {
                        match *color {
                            Rgb::RED => {
                                if score > &red_count { red_count = *score; }
                            },
                            Rgb::GREEN => {
                                if score > &green_count { green_count = *score; }
                            },
                            Rgb::BLUE => {
                                if score > &blue_count { blue_count = *score; }
                            }
                        }
                    })
                })
        }

        Ok(Self {
            id,
            red: red_count,
            green: green_count,
            blue: blue_count
        })
    }
}

impl Game {
    pub fn is_possible(&self, config: &Config) -> bool {
        self.red <= config.red &&
        self.green <= config.green &&
        self.blue <= config.blue
    }

    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

pub fn games<P>(filename: P) -> Vec<Game>
where P: AsRef<Path> {
    if let Ok(lines) = lines(filename) {
        lines
            .flatten()
            .filter_map(|line| Game::try_from(&line).ok())
            .collect()
    } else {
        vec![]
    }
}

pub fn possible_games<P>(filename: P, config: &Config) -> Vec<Game>
where P: AsRef<Path> {
    games(filename)
        .into_iter()
        .filter(|game| game.is_possible(config))
        .collect()
}

fn lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_is_possible() {
        let config = Config { red: 3, blue: 3, green: 3 };
        let possible_game = Game { id: 1, red: 3, blue: 3, green: 3 };
        let impossible_game = Game { id: 2, red: 4, blue: 3, green: 3 };
        assert!(possible_game.is_possible(&config));
        assert!(!impossible_game.is_possible(&config));
    }

    #[test]
    fn test_game_try_from_line() {
        let parseable = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string();
        let unparseable = "the dude abides".to_string();
        assert!(Game::try_from(&parseable).is_ok());
        assert!(Game::try_from(&unparseable).is_err());

        let game = Game::try_from(&parseable).unwrap();
        assert_eq!(game.id, 1);
        assert_eq!(game.red, 4);
        assert_eq!(game.green, 2);
        assert_eq!(game.blue, 6);
    }

    #[test]
    fn test_sample() {
        let config = Config { red: 12, green: 13, blue: 14 };
        let result = possible_games("input/sample.txt", &config);
        assert_eq!(result.len(), 3);
        let id_sum: u32 = result.iter().map(|g| g.id).sum();
        assert_eq!(id_sum, 8);
    }

    #[test]
    fn test_input() {
        let config = Config { red: 12, green: 13, blue: 14 };
        let result = possible_games("input/input.txt", &config);
        let id_sum: u32 = result.iter().map(|g| g.id).sum();
        assert_eq!(id_sum, 2810);
    }

    #[test]
    fn test_game_power() {
        let game = Game { id: 1, red: 2, green: 4, blue: 6 };
        assert_eq!(game.power(), 48);
    }

    #[test]
    fn test_sum_powers_from_sample() {
        let games = games("input/sample.txt");
        let sum = games
            .iter()
            .map(|game| game.power())
            .reduce(|acc, power| acc + power)
            .unwrap();
        assert_eq!(sum, 2286);
    }

    #[test]
    fn test_sum_powers_from_input() {
        let games = games("input/input.txt");
        let sum = games
            .iter()
            .map(|game| game.power())
            .reduce(|acc, power| acc + power)
            .unwrap();
        assert_eq!(sum, 2286);
    }
}
