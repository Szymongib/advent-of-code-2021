use std::fs;
use std::path::Path;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cucumber {
    East,
    South,
}

impl Cucumber {
    fn next_step(
        &self,
        (r, c): (usize, usize),
        matrix: &[Vec<Option<Cucumber>>],
    ) -> (usize, usize) {
        match self {
            Cucumber::East => right(r, c, matrix),
            Cucumber::South => down(r, c, matrix),
        }
    }
}

pub fn day_25_1<P: AsRef<Path>>(input_file: P) -> usize {
    let data = fs::read_to_string(input_file).unwrap();
    let mut cucumbers = parse_data(&data);

    let mut moved = true;
    let mut round = 0;

    while moved {
        moved = false;

        if move_cucumbers(Cucumber::East, &mut cucumbers) {
            moved = true
        }
        if move_cucumbers(Cucumber::South, &mut cucumbers) {
            moved = true
        }

        round += 1;
    }
    round
}

pub fn day_25_2<P: AsRef<Path>>(_input_file: P) -> String {
    String::from("Done!")
}

fn move_cucumbers(cuc_type: Cucumber, cucumbers: &mut Vec<Vec<Option<Cucumber>>>) -> bool {
    let mut remove_idx = vec![];
    let mut add_idx = vec![];
    let mut moved = false;

    for (r, row) in cucumbers.iter().enumerate() {
        for (c, cuc) in row.iter().enumerate() {
            if cuc.is_none() || cuc.unwrap() != cuc_type {
                continue;
            }

            let next_pos = cuc_type.next_step((r, c), cucumbers);
            if can_move_to(next_pos, cucumbers) {
                moved = true;
                remove_idx.push((r, c));
                add_idx.push(next_pos);
            };
        }
    }

    for rm_idx in remove_idx {
        cucumbers[rm_idx.0][rm_idx.1] = None;
    }
    for a_idx in add_idx {
        cucumbers[a_idx.0][a_idx.1] = Some(cuc_type);
    }

    moved
}

fn can_move_to((r, c): (usize, usize), matrix: &[Vec<Option<Cucumber>>]) -> bool {
    matrix[r][c].is_none()
}

fn down(r: usize, c: usize, matrix: &[Vec<Option<Cucumber>>]) -> (usize, usize) {
    if r == matrix.len() - 1 {
        (0, c)
    } else {
        (r + 1, c)
    }
}
fn right(r: usize, c: usize, matrix: &[Vec<Option<Cucumber>>]) -> (usize, usize) {
    if c == matrix[0].len() - 1 {
        (r, 0)
    } else {
        (r, c + 1)
    }
}

fn parse_data(data: &str) -> Vec<Vec<Option<Cucumber>>> {
    data.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'v' => Some(Cucumber::South),
                    '>' => Some(Cucumber::East),
                    _ => None,
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::day25::day_25_1;
    use crate::util::temp_file_with_content;

    const TEST_DATA: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn test() {
        let file = temp_file_with_content("day_25", TEST_DATA);

        assert_eq!(day_25_1(&file), 58);
    }
}
