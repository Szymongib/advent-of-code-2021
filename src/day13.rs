use std::cmp::Ordering;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::path::Path;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point(usize, usize);

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Point(x.parse().unwrap(), y.parse().unwrap()))
    }
}

struct Pattern {
    dots: HashSet<Point>,
}

impl Pattern {
    fn new(dots: Vec<Point>) -> Pattern {
        Pattern {
            dots: HashSet::from_iter(dots),
        }
    }

    fn fold_up(self, y: usize) -> Pattern {
        Pattern {
            dots: self
                .dots
                .into_iter()
                .filter_map(|point| match point.1.cmp(&y) {
                    Ordering::Less => Some(point),
                    Ordering::Equal => None,
                    Ordering::Greater => Some(Point(point.0, 2 * y - point.1)),
                })
                .collect(),
        }
    }

    fn fold_left(self, x: usize) -> Pattern {
        Pattern {
            dots: self
                .dots
                .into_iter()
                .filter_map(|point| match point.0.cmp(&x) {
                    Ordering::Less => Some(point),
                    Ordering::Equal => None,
                    Ordering::Greater => Some(Point(2 * x - point.0, point.1)),
                })
                .collect(),
        }
    }
}

impl ToString for Pattern {
    fn to_string(&self) -> String {
        let (max_x, max_y) = self.dots.iter().fold((0, 0), |(max_x, max_y), point| {
            (max_x.max(point.0), max_y.max(point.1))
        });

        let mut v = vec![vec!['.'; max_x + 1]; max_y + 1];
        for d in &self.dots {
            v[d.1][d.0] = '#';
        }
        let mut out = String::from("");
        for row in &v {
            out.push('\n');
            out.push_str(&row.iter().collect::<String>());
        }
        out
    }
}

enum Fold {
    Up(usize),
    Left(usize),
}

impl FromStr for Fold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fold_str = s.split(' ').nth(2).expect("expected 3 parts");
        let (direction, val) = fold_str.split_once('=').unwrap();

        match direction {
            "x" => Ok(Fold::Left(val.parse().unwrap())),
            "y" => Ok(Fold::Up(val.parse().unwrap())),
            _ => panic!("unexpected direction"),
        }
    }
}

pub fn day_13_1<P: AsRef<Path>>(input_file: P) -> usize {
    let (mut pattern, folds) = read_data(input_file);

    for f in folds.iter().take(1) {
        pattern = match f {
            Fold::Up(y) => pattern.fold_up(*y),
            Fold::Left(x) => pattern.fold_left(*x),
        };
    }

    pattern.dots.len()
}

pub fn day_13_2<P: AsRef<Path>>(input_file: P) -> String {
    let (mut pattern, folds) = read_data(input_file);

    for f in &folds {
        pattern = match f {
            Fold::Up(y) => pattern.fold_up(*y),
            Fold::Left(x) => pattern.fold_left(*x),
        };
    }

    pattern.to_string()
}

fn read_data<P: AsRef<Path>>(input_file: P) -> (Pattern, Vec<Fold>) {
    let data = std::fs::read_to_string(input_file).unwrap();
    let (points, folds) = data.split_once("\n\n").unwrap();

    let dots: Vec<Point> = points
        .lines()
        .map(|p| Point::from_str(p).unwrap())
        .collect();

    let folds: Vec<Fold> = folds
        .lines()
        .map(|fold| Fold::from_str(fold).unwrap())
        .collect();

    (Pattern::new(dots), folds)
}

#[cfg(test)]
mod test {
    use crate::day13::{day_13_1, day_13_2};
    use crate::util::temp_file_with_content;

    const TEST_DATA: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    const DAY_2_OUT: &str = "
#####
#...#
#...#
#...#
#####";

    #[test]
    fn test() {
        let file = temp_file_with_content("day_13_01", TEST_DATA);

        assert_eq!(day_13_1(&file), 17);
        assert_eq!(&day_13_2(&file), DAY_2_OUT);
    }
}
