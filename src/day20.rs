use std::fs;
use std::path::Path;

struct Image {
    pixels: Vec<Vec<usize>>,
    enhancement: Vec<usize>,
    padding: usize,
    runs: usize,
    switch_default: bool,
}

impl Image {
    fn new(s: &str, padding: usize) -> Image {
        let (enhancement, pixels) = s.split_once("\n\n").unwrap();

        let enhancement: Vec<usize> = enhancement
            .replace("\n", "") // For the purpose of test
            .chars()
            .map(pixel_to_val)
            .collect();

        let lines: Vec<&str> = pixels.lines().collect();

        let rows = lines.len();
        let cols = lines[0].len();

        let pixels_iter = lines.into_iter().map(|line| {
            let mut row = Vec::with_capacity(cols + 2 * padding);
            row.extend(vec![0; padding]);
            let nums = line.chars().map(pixel_to_val);
            row.extend(nums);
            row.extend(vec![0; padding]);
            row
        });

        let mut pixels = Vec::with_capacity(rows + 2 * padding);
        pixels.extend(vec![vec![0; cols + 2 * padding]; padding]);
        pixels.extend(pixels_iter);
        pixels.extend(vec![vec![0; cols + 2 * padding]; padding]);

        // If that is not the case we assume default is 0 otherwise we could end up with infinite ones.
        let switch_default = enhancement[0] == 1 && enhancement[511] == 0;

        Image {
            pixels,
            enhancement,
            padding,
            runs: 0,
            switch_default,
        }
    }

    fn enhance(&mut self) {
        let mut new_image = vec![vec![0; self.pixels[0].len()]; self.pixels.len()];

        let start = self.padding - 1;

        if self.switch_default {
            self.switch_padding(&mut new_image, self.padding);
        }

        for row in start..self.pixels.len() - start {
            for col in start..self.pixels.len() - start {
                let val = self.value_for_pixel(row, col);

                new_image[row][col] = self.enhancement[val];
            }
        }

        self.padding -= 1;
        self.pixels = new_image;
        self.runs += 1;
    }

    fn switch_padding(&self, new_image: &mut [Vec<usize>], padding: usize) {
        let default_val = if self.runs % 2 == 0 {
            self.enhancement[0]
        } else {
            self.enhancement[511]
        };
        let rows = new_image.len();
        let cols = new_image[0].len();

        for i in 0..padding {
            new_image[i].iter_mut().for_each(|v| *v = default_val);
            new_image[rows - 1 - i]
                .iter_mut()
                .for_each(|v| *v = default_val);
            for r in new_image.iter_mut() {
                r[i] = default_val;
                r[cols - 1 - i] = default_val;
            }
        }
    }

    fn value_for_pixel(&self, r: usize, c: usize) -> usize {
        let top = value_for_row(&self.pixels[r - 1][c - 1..c + 2], RowPosition::Top);
        let mid = value_for_row(&self.pixels[r][c - 1..c + 2], RowPosition::Middle);
        let down = value_for_row(&self.pixels[r + 1][c - 1..c + 2], RowPosition::Down);
        top + mid + down
    }
}

enum RowPosition {
    Top,
    Middle,
    Down,
}

impl RowPosition {
    fn to_base_power(&self) -> usize {
        match self {
            RowPosition::Top => 6,
            RowPosition::Middle => 3,
            RowPosition::Down => 0,
        }
    }
}

fn value_for_row(row: &[usize], pos: RowPosition) -> usize {
    let mut pow = 1;
    for _i in 0..pos.to_base_power() + 2 {
        pow *= 2;
    }

    let mut value = 0;
    for bin_val in row.iter().take(3) {
        value += bin_val * pow;
        pow /= 2;
    }
    value
}

pub fn day_20_1<P: AsRef<Path>>(input_file: P) -> usize {
    let data = fs::read_to_string(input_file).expect("failed to read input file");
    count_pixels(&data, 2)
}

pub fn day_20_2<P: AsRef<Path>>(input_file: P) -> usize {
    let data = fs::read_to_string(input_file).expect("failed to read input file");
    count_pixels(&data, 50)
}

fn count_pixels(data: &str, enhance_rounds: usize) -> usize {
    let mut image = Image::new(data, enhance_rounds + 1);

    for _i in 0..enhance_rounds {
        image.enhance();
    }

    image.pixels.iter().flat_map(|row| row.iter()).sum()
}

fn pixel_to_val(c: char) -> usize {
    match c {
        '.' => 0,
        '#' => 1,
        _ => unreachable!("invalid character: {}", c),
    }
}

#[cfg(test)]
mod test {
    use crate::day20::{day_20_1, day_20_2};
    use crate::util::temp_file_with_content;

    const TEST_DATA: &str =
        "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test() {
        let file = temp_file_with_content("day_20", TEST_DATA);

        assert_eq!(day_20_1(&file), 35);
        assert_eq!(day_20_2(&file), 3351);
    }
}
