use crate::util::read_lines_raw;
use std::path::Path;

pub fn day_9_1<P: AsRef<Path>>(input_file: P) -> u32 {
    let land_heights = read_land_heights(input_file);

    let mut sum = 0;
    for r in 0..land_heights.len() {
        for c in 0..land_heights[0].len() {
            let val = land_heights[r][c];
            let neighbours = get_neighbours(r, c, &land_heights);

            if val < *neighbours.iter().min().unwrap() {
                sum += val + 1;
            }
        }
    }

    sum
}

pub fn day_9_2<P: AsRef<Path>>(input_file: P) -> u32 {
    let land_heights = read_land_heights(input_file);

    let mut visited = vec![vec![false; land_heights[0].len()]; land_heights.len()];
    let mut basin_sizes = vec![];

    for r in 0..land_heights.len() {
        for c in 0..land_heights[0].len() {
            let size = check_basin_size(r, c, &land_heights, &mut visited);
            if size > 0 {
                basin_sizes.push(size);
            }
        }
    }

    basin_sizes.sort_unstable();

    basin_sizes.iter().skip(basin_sizes.len() - 3).product()
}

fn read_land_heights<P: AsRef<Path>>(input_file: P) -> Vec<Vec<u32>> {
    read_lines_raw(input_file)
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).expect("failed to parse to digit"))
                .collect()
        })
        .collect()
}

fn get_neighbours(r: usize, c: usize, matrix: &[Vec<u32>]) -> Vec<u32> {
    get_neighbour_pos(r, c, matrix)
        .iter()
        .map(|(nr, nc)| matrix[*nr][*nc])
        .collect()
}

fn check_basin_size(r: usize, c: usize, matrix: &[Vec<u32>], visited: &mut [Vec<bool>]) -> u32 {
    if matrix[r][c] == 9 || visited[r][c] {
        return 0;
    }

    visited[r][c] = true;

    let neighbours = get_neighbour_pos(r, c, matrix);

    let mut sum = 1;
    for (nr, nc) in neighbours {
        sum += check_basin_size(nr, nc, matrix, visited);
    }

    sum
}

fn get_neighbour_pos(r: usize, c: usize, matrix: &[Vec<u32>]) -> Vec<(usize, usize)> {
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

#[cfg(test)]
mod test {
    use crate::day09::{day_9_1, day_9_2};
    use crate::util::temp_file_with_content;

    const TEST_DATA: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test() {
        let file = temp_file_with_content("day_09_01", TEST_DATA);

        assert_eq!(day_9_1(&file), 15);
        assert_eq!(day_9_2(&file), 1134);
    }
}
