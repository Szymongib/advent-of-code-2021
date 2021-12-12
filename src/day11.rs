use crate::util::read_lines_raw;
use std::path::Path;

pub fn day_11_1<P: AsRef<Path>>(input_file: P) -> usize {
    let mut octo_light = read_octo_energy(input_file);

    for _i in 0..100 {
        octo_light.start_step();
    }

    octo_light.total_flashes
}

pub fn day_11_2<P: AsRef<Path>>(input_file: P) -> usize {
    let mut octo_light = read_octo_energy(input_file);

    let octo_count = octo_light.energy.len() * octo_light.energy[0].len();

    let mut rounds = 0;
    while octo_light.step_flashes < octo_count {
        octo_light.start_step();
        rounds += 1;
    }
    rounds
}

fn read_octo_energy<P: AsRef<Path>>(input_file: P) -> OctoLight {
    let energy = read_lines_raw(input_file)
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| (c.to_digit(10).expect("failed to parse to digit"), false))
                .collect()
        })
        .collect();
    OctoLight::new(energy)
}

struct OctoLight {
    energy: Vec<Vec<(u32, bool)>>,
    total_flashes: usize,
    step_flashes: usize,
}

impl OctoLight {
    fn new(energy: Vec<Vec<(u32, bool)>>) -> OctoLight {
        OctoLight {
            energy,
            total_flashes: 0,
            step_flashes: 0,
        }
    }

    fn start_step(&mut self) {
        self.step_flashes = 0;
        let mut flashes = vec![];

        for (r, row) in self.energy.iter_mut().enumerate() {
            for (c, val) in row.iter_mut().enumerate() {
                val.0 += 1;
                if val.0 > 9 {
                    flashes.push((r, c))
                }
            }
        }

        for (r, c) in flashes {
            self.flash(r, c);
        }

        self.total_flashes += self.step_flashes;
        self.finish_step()
    }

    fn flash(&mut self, r: usize, c: usize) {
        if self.energy[r][c].1 {
            return;
        }

        self.energy[r][c].1 = true;
        self.energy[r][c].0 = 0;
        self.step_flashes += 1;

        let neighbour_pos = get_neighbour_pos_with_diagonal(r, c, &self.energy);

        for (r, c) in neighbour_pos {
            self.increase(r, c);
        }
    }

    fn increase(&mut self, r: usize, c: usize) {
        if self.energy[r][c].1 {
            return;
        }

        self.energy[r][c].0 += 1;
        if self.energy[r][c].0 > 9 {
            self.flash(r, c);
        }
    }

    fn finish_step(&mut self) {
        for row in self.energy.iter_mut() {
            for val in row {
                val.1 = false;
            }
        }
    }
}

fn get_neighbour_pos_with_diagonal(
    r: usize,
    c: usize,
    matrix: &[Vec<(u32, bool)>],
) -> Vec<(usize, usize)> {
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
    // diagonal
    if r > 0 && c > 0 {
        neighbours.push((r - 1, c - 1));
    }
    if r > 0 && c < matrix[0].len() - 1 {
        neighbours.push((r - 1, c + 1));
    }
    if r < matrix.len() - 1 && c < matrix[0].len() - 1 {
        neighbours.push((r + 1, c + 1));
    }
    if r < matrix.len() - 1 && c > 0 {
        neighbours.push((r + 1, c - 1));
    }

    neighbours
}

#[cfg(test)]
mod test {
    use crate::day11::{day_11_1, day_11_2};
    use crate::util::temp_file_with_content;

    const TEST_DATA: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test() {
        let file = temp_file_with_content("day_11_01", TEST_DATA);

        assert_eq!(day_11_1(&file), 1656);
        assert_eq!(day_11_2(&file), 195);
    }
}
