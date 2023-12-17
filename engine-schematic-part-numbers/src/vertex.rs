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

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Data {
    Number(u32),
    Symbol(char),
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Vertex {
    pub data: Data,
    pub y: usize,
    pub min_x: usize,
    pub max_x: usize,
}

impl Vertex {
    pub fn number(number: u32, y: usize, min_x: usize) -> Self {
        let n = format!("{number}");
        Self {
            data: Data::Number(number),
            y,
            min_x,
            max_x: min_x + n.len() - 1,
        }
    }

    pub fn symbol(symbol: char, y: usize, min_x: usize) -> Self {
        Self {
            data: Data::Symbol(symbol),
            y,
            min_x,
            max_x: min_x,
        }
    }

    pub fn adjacent_points(&self) -> HashSet<Point> {
        let mut adjacents: HashSet<Point> = HashSet::new();
        let min_y = if self.y.checked_sub(1).is_some() { self.y - 1 } else { self.y };
        let max_y = self.y + 1;
        let min_x = if self.min_x.checked_sub(1).is_some() {
            self.min_x - 1
        } else {
            self.min_x
        };
        let max_x = self.max_x + 1;
        (min_x..=max_x)
            .for_each(|x| {
                (min_y..=max_y).for_each(|y| {
                    adjacents.insert(Point::new(x, y));
                })
            });

        adjacents
    }

    pub fn occupied_points(&self) -> HashSet<Point> {
        let mut points: HashSet<Point> = HashSet::new();
        (self.min_x..=self.max_x)
            .for_each(|x| {
                points.insert(Point::new(x, self.y));
            });
        points
    }

    pub fn is_adjacent_to(&self, other: &Vertex) -> bool {
        let my_ap = self.adjacent_points();
        let other_ap = other.occupied_points();
        !my_ap.is_disjoint(&other_ap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_adjacent_points() {
        let number = Vertex::number(112, 1, 1);
        assert_eq!(number.max_x, 3);
        let adj_pts = number.adjacent_points();
        assert_eq!(adj_pts.len(), 15);
    }

    #[test]
    fn test_number_on_border_adjacent_points() {
        let number = Vertex::number(12, 0, 0);
        let adj_pts = number.adjacent_points();
        assert_eq!(adj_pts.len(), 6);
    }

    #[test]
    fn test_adjacent_numbers() {
        let n1 = Vertex::number(112, 1, 1);
        let n2 = Vertex::number(345, 2, 1);

        assert!(n1.is_adjacent_to(&n2));
    }

    #[test]
    fn test_non_adjacent_numbers() {
        let n1 = Vertex::number(112, 1, 1);
        let n2 = Vertex::number(112, 3, 1);

        assert!(!n1.is_adjacent_to(&n2));
    }

    #[test]
    fn test_symbol_adjacent_to_number() {
        let s = Vertex::symbol('#', 0, 0);
        let n = Vertex::number(3, 0, 1);
        assert!(s.is_adjacent_to(&n));
    }

    #[test]
    fn test_symbol_adjacent_to_number_same_line() {
        let s = Vertex::symbol('*', 4, 3);
        let n = Vertex::number(617, 4, 0);
        assert!(s.is_adjacent_to(&n));

        let s = Vertex::symbol('*', 4, 2);
        let n = Vertex::number(617, 4, 3);
        assert!(s.is_adjacent_to(&n));
    }
}
