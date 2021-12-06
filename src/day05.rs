use crate::util::read_lines;
use std::path::Path;
use std::str::FromStr;

enum Orientation {
    Horizontal,
    Vertical,
    Diagonal,
}

struct Line {
    p1: (usize, usize),
    p2: (usize, usize),
    orientation: Orientation,
}

impl Line {
    fn determine_orientation(p1: (usize, usize), p2: (usize, usize)) -> Orientation {
        if p1.1 == p2.1 {
            Orientation::Horizontal
        } else if p1.0 == p2.0 {
            Orientation::Vertical
        } else {
            Orientation::Diagonal
        }
    }

    fn max_x(&self) -> usize {
        self.p1.0.max(self.p2.0)
    }
    fn min_x(&self) -> usize {
        self.p1.0.min(self.p2.0)
    }

    fn max_y(&self) -> usize {
        self.p1.1.max(self.p2.1)
    }
    fn min_y(&self) -> usize {
        self.p1.1.min(self.p2.1)
    }

    fn apply_line(&self, diagram: &mut [Vec<usize>], ignore_diag: bool) {
        match self.orientation {
            Orientation::Horizontal => {
                for i in self.min_x()..self.max_x() + 1 {
                    diagram[self.p1.1][i] += 1;
                }
            }
            Orientation::Vertical => {
                for i in self.min_y()..self.max_y() + 1 {
                    diagram[i][self.p1.0] += 1;
                }
            }
            Orientation::Diagonal => {
                if ignore_diag {
                    return;
                }
                // This could probably be simplified...
                if self.p1.0 <= self.p2.0 && self.p1.1 <= self.p2.1 {
                    // Down right
                    for i in 0..(self.p2.0 + 1 - self.p1.0) {
                        diagram[self.p1.1 + i][self.p1.0 + i] += 1
                    }
                } else if self.p1.0 >= self.p2.0 && self.p1.1 >= self.p2.1 {
                    // Down left
                    for i in 0..(self.p1.0 + 1 - self.p2.0) {
                        diagram[self.p1.1 - i][self.p1.0 - i] += 1
                    }
                } else if self.p1.0 >= self.p2.0 && self.p1.1 < self.p2.1 {
                    // Up left
                    for i in 0..(self.p1.0 + 1 - self.p2.0) {
                        diagram[self.p1.1 + i][self.p1.0 - i] += 1
                    }
                } else if self.p1.0 < self.p2.0 && self.p1.1 > self.p2.1 {
                    // Up right
                    for i in 0..(self.p2.0 + 1 - self.p1.0) {
                        diagram[self.p1.1 - i][self.p1.0 + i] += 1
                    }
                }
            }
        }
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<&str> = s.split("->").collect();

        let p1 = point_from_str(points[0]);
        let p2 = point_from_str(points[1]);

        Ok(Line {
            p1,
            p2,
            orientation: Line::determine_orientation(p1, p2),
        })
    }
}

fn point_from_str(s: &str) -> (usize, usize) {
    let cords: Vec<usize> = s
        .split(',')
        .map(|pos| pos.trim().parse().expect("expected coordinates"))
        .collect();

    (cords[0], cords[1])
}

pub fn day_5_1<P: AsRef<Path>>(input_file: P) -> usize {
    let lines: Vec<Line> = read_lines(input_file);

    let mut diagram = init_diagram(&lines);

    for l in lines {
        l.apply_line(&mut diagram, true);
    }

    count_more_than_2(&diagram)
}

pub fn day_5_2<P: AsRef<Path>>(input_file: P) -> usize {
    let lines: Vec<Line> = read_lines(input_file);

    let mut diagram = init_diagram(&lines);

    for l in lines {
        l.apply_line(&mut diagram, false);
    }

    count_more_than_2(&diagram)
}

fn init_diagram(lines: &[Line]) -> Vec<Vec<usize>> {
    let (max_x, max_y) = lines
        .iter()
        .map(|l| (l.max_x(), l.max_y()))
        .fold((0, 0), |(mx, my), (cx, cy)| (mx.max(cx), my.max(cy)));
    vec![vec![0; max_x + 1]; max_y + 1]
}

fn count_more_than_2(diagram: &[Vec<usize>]) -> usize {
    diagram.iter().fold(0, |acc, row| {
        acc + row.iter().fold(
            0,
            |row_acc, val| {
                if *val >= 2 {
                    row_acc + 1
                } else {
                    row_acc
                }
            },
        )
    })
}

#[cfg(test)]
mod test {
    use crate::day05::{day_5_1, day_5_2};
    use crate::util::temp_file_with_content;

    const TEST_DATA: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test() {
        let file = temp_file_with_content("day_05_01", TEST_DATA);

        assert_eq!(day_5_1(&file), 5);
        assert_eq!(day_5_2(&file), 12);
    }
}
