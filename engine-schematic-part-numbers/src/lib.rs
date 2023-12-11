use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub struct PartNumber {
    pub number: u32,
    pub start_pos: Point,
    pub end_pos: Point,
}

#[derive(Default)]
pub struct LineAnalysis {
    pub numbers: Vec<PartNumber>,
    pub symbol_positions: Vec<Point>,
}

pub fn part_numbers(input: Vec<String>) -> Vec<PartNumber> {
    let mut all_numbers: Vec<PartNumber> = vec![];
    let mut sym_pos: HashSet<Point> = HashSet::new();
    input
        .iter()
        .enumerate()
        .for_each(|(y, line)| {
            let mut analysis = analyze_line(line, y);
            all_numbers.append(&mut analysis.numbers);

            analysis
                .symbol_positions
                .into_iter()
                .for_each(|p| {
                    sym_pos.insert(p);
                });
        });

    all_numbers
        .into_iter()
        .filter(|pn| {
            adjacent_points(pn)
                .iter()
                .any(|p| sym_pos.contains(p))
        })
        .collect()
}

fn analyze_line(line: &str, y: usize) -> LineAnalysis {
    let mut analysis = LineAnalysis::default();
    let reg = Regex::new(r"(\d+)|([^\.]{1})").unwrap();
    reg.find_iter(line)
        .for_each(|m| {
            if let Ok(number) = m.as_str().parse() {
                let pn = PartNumber {
                    number,
                    start_pos: Point::new(m.start(), y),
                    end_pos: Point::new(m.end() - 1, y)
                };
                analysis.numbers.push(pn);
            } else {
                let symbol_pos = Point::new(m.start(), y);
                analysis.symbol_positions.push(symbol_pos);
            }
        });
    analysis
}

fn adjacent_points(part_number: &PartNumber) -> Vec<Point> {
    let start_x = part_number.start_pos.x;
    let end_x = part_number.end_pos.x;
    let y = part_number.start_pos.y;

    let min_y = if y.checked_sub(1).is_some() {
        y - 1
    } else {
        y
    };
    let max_y = y + 1;
    let min_x = if start_x.checked_sub(1).is_some() {
        start_x - 1
    } else {
        start_x
    };
    let max_x = end_x + 1;
    let mut adjacents = vec![];
    (min_x..=max_x)
        .for_each(|x| {
            (min_y..=max_y).for_each(|y| {
                adjacents.push(Point::new(x, y))
            })
        });

    adjacents
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::File,
        io::{self, BufRead},
        path::Path,
    };

    fn read_input<P>(filename: P) -> Vec<String>
    where P: AsRef<Path> {
        if let Ok(lines) = lines(filename) {
            lines
                .flatten()
                .collect()
        } else {
            vec![]
        }

    }

    fn lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path> {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    #[test]
    fn test_analyze_line_without_symbols() {
        let analysis = analyze_line("467..114..", 0);
        assert_eq!(analysis.numbers.len(), 2);
        assert_eq!(analysis.symbol_positions.len(), 0);

        let first = analysis.numbers.first().unwrap();
        let second = analysis.numbers.get(1).unwrap();
        assert_eq!(first.number, 467);
        assert_eq!(first.start_pos, Point::new(0, 0));
        assert_eq!(first.end_pos, Point::new(2, 0));

        assert_eq!(second.number, 114);
        assert_eq!(second.start_pos, Point::new(5, 0));
        assert_eq!(second.end_pos, Point::new(7, 0));
    }

    #[test]
    fn test_analyze_line_with_symbols() {
        let analysis = analyze_line("467#.114.+", 4);
        assert_eq!(analysis.numbers.len(), 2);
        assert_eq!(analysis.symbol_positions.len(), 2);

        let first = analysis.numbers.first().unwrap();
        let second = analysis.numbers.get(1).unwrap();
        assert_eq!(first.number, 467);
        assert_eq!(first.start_pos, Point::new(0, 4));
        assert_eq!(first.end_pos, Point::new(2, 4));

        assert_eq!(second.number, 114);
        assert_eq!(second.start_pos, Point::new(5, 4));
        assert_eq!(second.end_pos, Point::new(7, 4));

        let sym1 = analysis.symbol_positions.first().unwrap();
        let sym2 = analysis.symbol_positions.get(1).unwrap();
        assert_eq!(sym1, &Point::new(3, 4));
        assert_eq!(sym2, &Point::new(9, 4));
    }

    #[test]
    fn test_sum_part_numbers_from_sample() {
        let sample_input = read_input("input/sample.txt");
        let part_numbers = part_numbers(sample_input);
        let sum = part_numbers
            .iter()
            .map(|pn| {
                pn.number
            })
            .reduce(|acc, n| acc + n)
            .unwrap();
        assert_eq!(sum, 4361);
    }

    #[test]
    fn test_sum_part_numbers_from_input() {
        let sample_input = read_input("input/input.txt");
        let part_numbers = part_numbers(sample_input);
        let sum = part_numbers
            .iter()
            .map(|pn| {
                pn.number
            })
            .reduce(|acc, n| acc + n)
            .unwrap();
        assert_eq!(sum, 546563);
    }
}
