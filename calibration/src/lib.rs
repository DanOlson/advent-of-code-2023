use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn calibration_value(path: &str) -> u32 {
    let mut retval = 0;
    if let Ok(lines) = lines(path) {
        lines.flatten().for_each(|line| {
            if let Some(calib_val) = get_value(&line) {
                // println!("{line} - {calib_val}");
                retval += calib_val;
            }
        });
    }
    retval
}

const PATTERNS: [&str; 18] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9"
];

fn get_value(line: &str) -> Option<u32> {
    let a = first_match(line);
    let b = last_match(line);

    a.map(|val| val * 10 + b.unwrap())
}

fn first_match(line: &str) -> Option<u32> {
    PATTERNS
        .iter()
        .filter_map(|p| {
            line.find(p)
                .map(|index| {
                    let value = value_of(p).unwrap();
                    Match { index, value }
                })
        })
        .min_by(|m1, m2| m1.index.cmp(&m2.index))
        .map(|m| m.value)
}

fn value_of(pattern: &str) -> Option<u32> {
    match pattern {
        "one" | "1" => Some(1),
        "two" | "2" => Some(2),
        "three" | "3" => Some(3),
        "four" | "4" => Some(4),
        "five" | "5" => Some(5),
        "six" | "6" => Some(6),
        "seven" | "7" => Some(7),
        "eight" | "8" => Some(8),
        "nine" | "9" => Some(9),
        _ => None
    }
}

fn last_match(line: &str) -> Option<u32> {
    PATTERNS
        .iter()
        .filter_map(|p| {
            line.rfind(p)
                .map(|index| {
                    let value = value_of(p).unwrap();
                    Match { index, value }
                })
        })
        .max_by(|m1, m2| m1.index.cmp(&m2.index))
        .map(|m| m.value)
}

struct Match {
    pub value: u32,
    pub index: usize
}

fn lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let result = calibration_value("input/sample.txt");
        assert_eq!(result, 142);
    }

    #[test]
    fn test_input() {
        let result = calibration_value("input/input.txt");
        assert_eq!(result, 55488);
    }

    #[test]
    fn test_with_numeric_and_words() {
        let result = calibration_value("input/sample2.txt");
        assert_eq!(result, 443);
    }
}
