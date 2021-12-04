use std::fs;
use std::path::Path;

struct Board {
    pub board: Vec<Vec<(u32, bool)>>,
}

impl Board {
    fn new_from_raw(data: &str) -> Board {
        let board: Vec<Vec<(u32, bool)>> = data
            .split("\n")
            .map(|line| {
                line.split(' ')
                    .filter(|elem| *elem != " " && *elem != "")
                    .map(|num| (num.parse().expect("failed to parse board element"), false))
                    .collect()
            })
            .collect();

        Board { board }
    }

    fn mark_num(&mut self, num: u32) {
        self.board.iter_mut().for_each(|row| {
            for record in row.iter_mut() {
                if record.0 == num {
                    record.1 = true
                }
            }
        });
    }

    fn calc_score(&self) -> Option<u32> {
        let mut sum = 0;
        let mut won = false;

        for r in &self.board {
            let mut finished = 0;
            for (elem, marked) in r {
                if *marked {
                    finished += 1;
                } else {
                    sum += elem;
                }
            }
            if finished == r.len() {
                won = true;
            }
        }

        // Exit early if row complete
        if won {
            return Some(sum);
        }

        for i in 0..self.board.len() {
            let mut finished = 0;
            for k in 0..self.board[0].len() {
                if self.board[k][i].1 == true {
                    finished += 1;
                }
            }
            if finished == self.board.len() {
                return Some(sum);
            }
        }
        None
    }
}

pub fn day_4_1<P: AsRef<Path>>(input_file: P) -> u32 {
    let (draws, mut boards) = parse_input(input_file);

    for draw in draws {
        for board in boards.iter_mut() {
            board.mark_num(draw);
            if let Some(score) = board.calc_score() {
                return score * draw;
            }
        }
    }

    0
}

pub fn day_4_2<P: AsRef<Path>>(input_file: P) -> u32 {
    let (draws, mut boards) = parse_input(input_file);

    let mut scores: Vec<u32> = Vec::with_capacity(boards.len());

    let mut draw_idx = 0;
    while draw_idx < draws.len() && !boards.is_empty() {
        let draw = draws[draw_idx];
        let mut rem = Vec::new();
        for i in 0..boards.len() {
            boards[i].mark_num(draw);
            if let Some(score) = boards[i].calc_score() {
                rem.push(i);
                scores.push(score * draw);
            }
        }

        // Remove bards that already won
        boards = boards
            .into_iter()
            .enumerate()
            .filter(|(i, _)| !rem.contains(i))
            .map(|(_, b)| b)
            .collect();
        draw_idx += 1;
    }

    *scores.last().expect("expected last board score")
}

fn parse_input<P: AsRef<Path>>(input_file: P) -> (Vec<u32>, Vec<Board>) {
    let data = fs::read_to_string(input_file).expect("read data from file");

    let segments: Vec<&str> = data.split("\n\n").collect();
    let draws: Vec<u32> = segments[0]
        .split(',')
        .map(|s| s.parse().expect("failed to parse number"))
        .collect();

    let boards: Vec<Board> = segments
        .into_iter()
        .skip(1)
        .map(|segment| Board::new_from_raw(segment))
        .collect();

    (draws, boards)
}

#[cfg(test)]
mod test {
    use crate::day04::{day_4_1, day_4_2};
    use crate::util::temp_file_with_content;

    const TEST_DATA: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test() {
        let file = temp_file_with_content("day_04_01", TEST_DATA);

        assert_eq!(day_4_1(&file), 4512);
        assert_eq!(day_4_2(&file), 1924)
    }
}
