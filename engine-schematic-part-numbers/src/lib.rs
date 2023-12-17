use regex::Regex;
use std::collections::{HashSet, HashMap};

mod vertex;

use vertex::{Data, Vertex};

pub fn part_numbers(input: Vec<String>) -> Vec<u32> {
    build_adjacency_list(input)
        .iter()
        .filter_map(|(vertex, adjacents)| {
            // find numbers adjacent to symbols
            match vertex.data {
                Data::Number(n) => {
                    if adjacents.iter().any(|v| matches!(v.data, Data::Symbol(_))) {
                        Some(n)
                    } else {
                        None
                    }
                },
                Data::Symbol(_s) => None
            }
        })
        .collect::<Vec<u32>>()
}

pub fn gear_ratios(input: Vec<String>) -> Vec<u32> {
    build_adjacency_list(input)
        .iter()
        .filter_map(|(vertex, adjacents)| {
            match vertex.data {
                Data::Number(_n) => None,
                Data::Symbol(_s) => {
                    let iter = adjacents.iter();
                    if iter.len() == 2 && iter.clone().all(|v| matches!(v.data, Data::Number(_))) {
                        let gear_ratio = iter.map(|n| {
                            if let Data::Number(num) = n.data { num } else { 0 }
                        }).product::<u32>();
                        Some(gear_ratio)
                    } else {
                        None
                    }
                }
            }
        })
        .collect::<Vec<u32>>()
}

fn build_adjacency_list(input: Vec<String>) -> HashMap<Vertex, HashSet<Vertex>> {
    let mut verts_by_line_no: HashMap<usize, Vec<Vertex>> = HashMap::new();
    let mut adj_list: HashMap<Vertex, HashSet<Vertex>> = HashMap::new();
    input
        .iter()
        .enumerate()
        .for_each(|(y, line)| {
            let analysis = analyze_line(line, y);
            let iter = analysis.iter();
            let mut with_offset = iter.clone();
            with_offset.next();
            for (a, b) in iter.zip(with_offset) {
                if a.is_adjacent_to(b) {
                    adj_list.entry(*a).or_default().insert(*b);
                    adj_list.entry(*b).or_default().insert(*a);
                }
            }
            if y > 0 {
                let last_verts = verts_by_line_no.get(&(y - 1)).unwrap();
                analysis
                    .iter()
                    .for_each(|v| {
                        last_verts.iter().for_each(|lv| {
                            if v.is_adjacent_to(lv) {
                                adj_list.entry(*lv).or_default().insert(*v);
                                adj_list.entry(*v).or_default().insert(*lv);
                            }
                        })
                    });
            }
            verts_by_line_no.insert(y, analysis);
        });
        adj_list
}

fn analyze_line(line: &str, y: usize) -> Vec<Vertex> {
    let mut analysis = vec![];
    let reg = Regex::new(r"(\d+)|([^.])").unwrap();
    reg.find_iter(line)
        .for_each(|m| {
            if let Ok(number) = m.as_str().parse() {
                let n = Vertex::number(number, y, m.start());
                analysis.push(n);
            } else {
                let symbol = m.as_str().chars().next().unwrap();
                let s = Vertex::symbol(symbol, y, m.start());
                analysis.push(s);
            }
        });
    analysis
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
    fn test_sum_part_numbers_from_sample() {
        let sample_input = read_input("input/sample.txt");
        let part_numbers = part_numbers(sample_input);
        let sum: u32 = part_numbers
            .iter()
            .sum();

        assert_eq!(sum, 4361);
    }

    #[test]
    fn test_sum_part_numbers_from_input() {
        let sample_input = read_input("input/input.txt");
        let part_numbers = part_numbers(sample_input);
        let sum: u32 = part_numbers
            .iter()
            .sum();

        assert_eq!(sum, 546563);
    }

    #[test]
    fn test_multiple_adjacencies_in_one_line() {
        let sample_input = vec!["..99*.99*".to_string()];
        let part_numbers = part_numbers(sample_input);
        let sum: u32 = part_numbers.iter().sum();
        assert_eq!(sum, 198);
    }

    #[test]
    fn test_multiple_runon_adjacencies_in_one_line() {
        let sample_input = vec!["..99*/99.".to_string()];
        let part_numbers = part_numbers(sample_input);
        let sum: u32 = part_numbers.iter().sum();
        assert_eq!(sum, 198);
    }

    #[test]
    fn test_analyze_line() {
        let analysis = analyze_line("..99**99..", 0);
        assert_eq!(analysis.len(), 4);
        let first_number = analysis.iter().find(|v| matches!(v.data, Data::Number(99)) && v.min_x == 2).unwrap();
        let second_number = analysis.iter().find(|v| matches!(v.data, Data::Number(99)) && v.min_x == 6).unwrap();
        let first_symbol = analysis.iter().find(|v| matches!(v.data, Data::Symbol('*')) && v.min_x == 4).unwrap();
        let second_symbol = analysis.iter().find(|v| matches!(v.data, Data::Symbol('*')) && v.min_x == 5).unwrap();
        assert!(first_number.is_adjacent_to(first_symbol));
        assert!(first_symbol.is_adjacent_to(first_number));
        assert!(second_symbol.is_adjacent_to(second_number));
        assert!(second_number.is_adjacent_to(second_symbol));
    }

    #[test]
    fn test_gear_ratio_with_sample() {
        let sample_input = read_input("input/sample.txt");
        let sum: u32 = gear_ratios(sample_input).iter().sum();
        assert_eq!(sum, 467835);
    }

    #[test]
    fn test_gear_ratios_with_input() {
        let input = read_input("input/input.txt");
        let sum: u32 = gear_ratios(input).iter().sum();
        assert_eq!(sum, 91031374);
    }
}
