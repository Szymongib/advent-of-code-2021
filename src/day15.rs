use crate::util::read_lines_raw;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::path::Path;

pub fn day_15_1<P: AsRef<Path>>(input_file: P) -> usize {
    let risk_levels = read_risk_levels(input_file);

    dijkstra(risk_levels)
}

pub fn day_15_2<P: AsRef<Path>>(input_file: P) -> usize {
    let risk_levels = read_risk_levels(input_file);
    let risk_levels = transform_x5(risk_levels);

    dijkstra(risk_levels)
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct CavePosition {
    cost: usize,
    position: (usize, usize),
}

// Order based on cost for the sake of priority queue.
impl Ord for CavePosition {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for CavePosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(risk_levels: Vec<Vec<usize>>) -> usize {
    let end_pos = (risk_levels.len() - 1, risk_levels[0].len() - 1);

    let mut total_risk = vec![vec![usize::MAX; risk_levels[0].len()]; risk_levels.len()];
    total_risk[0][0] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(CavePosition {
        cost: 0,
        position: (0, 0),
    });

    while let Some(CavePosition {
        cost,
        position: pos,
    }) = heap.pop()
    {
        if pos == end_pos {
            return cost;
        }

        if cost > total_risk[pos.0][pos.1] {
            continue;
        }

        for next_pos in get_neighbour_pos(pos.0, pos.1, &total_risk) {
            let next = CavePosition {
                cost: cost + risk_levels[next_pos.0][next_pos.1],
                position: next_pos,
            };

            if next.cost < total_risk[next_pos.0][next_pos.1] {
                heap.push(next);
                total_risk[next_pos.0][next_pos.1] = next.cost;
            }
        }
    }

    unreachable!("failed to find path to the end position");
}

fn get_neighbour_pos(r: usize, c: usize, matrix: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::with_capacity(4);

    if r > 0 {
        neighbours.push((r - 1, c));
    }
    if r < matrix.len() - 1 {
        neighbours.push((r + 1, c));
    }
    if c > 0 {
        neighbours.push((r, c - 1));
    }
    if c < matrix[0].len() - 1 {
        neighbours.push((r, c + 1));
    }

    neighbours
}

fn read_risk_levels<P: AsRef<Path>>(input_file: P) -> Vec<Vec<usize>> {
    read_lines_raw(input_file)
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).expect("failed to parse to digit") as usize)
                .collect()
        })
        .collect()
}

fn transform_x5(mut matrix: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let org_rows = matrix.len();
    let org_cols = matrix[0].len();

    matrix.resize(org_rows * 5, vec![]);

    for i in 1..5 {
        for k in 0..org_rows {
            let row = matrix[k + (org_rows * (i - 1))]
                .clone()
                .into_iter()
                .map(|d| truncate_value((d + 1) % 10))
                .collect();
            matrix[k + i * org_rows] = row;
        }
    }

    for row in matrix.iter_mut() {
        row.resize(org_cols * 5, 0);

        for i in 1..5 {
            for k in 0..org_cols {
                let val = truncate_value((row[k + (org_cols * (i - 1))] + 1) % 10);
                row[k + i * org_cols] = val;
            }
        }
    }

    matrix
}

fn truncate_value(value: usize) -> usize {
    match value {
        0 => 1,
        val => val,
    }
}

#[cfg(test)]
mod test {
    use crate::day15::{day_15_1, day_15_2, transform_x5};
    use crate::util::temp_file_with_content;

    const TEST_DATA: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    fn read(s: &str) -> Vec<Vec<usize>> {
        s.lines()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).expect("failed to parse to digit") as usize)
                    .collect()
            })
            .collect()
    }

    #[test]
    fn test() {
        let file = temp_file_with_content("day_15_01", TEST_DATA);
        assert_eq!(day_15_1(&file), 40);
        assert_eq!(day_15_2(&file), 315);
    }
}
