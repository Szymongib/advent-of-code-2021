use std::fs;
use std::path::Path;

pub fn day_7_1<P: AsRef<Path>>(input_file: P) -> i32 {
    let mut crab_pos: Vec<i32> = fs::read_to_string(input_file)
        .expect("failed to read data from file")
        .split(',')
        .map(|n| n.parse().expect("failed to parse as number"))
        .collect();

    crab_pos.sort_unstable();
    let median = crab_pos[crab_pos.len() / 2 - 1];

    crab_pos
        .iter()
        .fold(0, |acc, pos| acc + (median - *pos).abs())
}

pub fn day_7_2<P: AsRef<Path>>(input_file: P) -> i32 {
    let crab_pos: Vec<i32> = fs::read_to_string(input_file)
        .expect("failed to read data from file")
        .split(',')
        .map(|n| n.parse().expect("failed to parse as number"))
        .collect();

    let mean = crab_pos.iter().sum::<i32>() as f64 / crab_pos.len() as f64;

    // Simply rounding the mean did not seem to work in case of mean ending with .501
    // so we ceil and floor and pick the best result from that.
    calc_fuel_increasing(mean.ceil() as i32, &crab_pos)
        .min(calc_fuel_increasing(mean.floor() as i32, &crab_pos))
}

fn calc_fuel_increasing(target_pos: i32, positions: &[i32]) -> i32 {
    positions.iter().fold(0, |acc, pos| {
        let n = (target_pos - *pos).abs();
        acc + (n * (n + 1)) / 2
    })
}

#[cfg(test)]
mod test {
    use crate::day07::{day_7_1, day_7_2};
    use crate::util::temp_file_with_content;

    const TEST_DATA: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test() {
        let file = temp_file_with_content("day_07_01", TEST_DATA);

        assert_eq!(day_7_1(&file), 37);
        assert_eq!(day_7_2(&file), 168);
    }
}
