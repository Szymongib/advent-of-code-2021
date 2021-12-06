use std::fs;
use std::path::Path;

pub fn day_6_1<P: AsRef<Path>>(input_file: P) -> u64 {
    let fish_timers = read_lantern_fish_timers(input_file);
    simulate_lantern_fish_population(fish_timers, 80)
}

pub fn day_6_2<P: AsRef<Path>>(input_file: P) -> u64 {
    let fish_timers = read_lantern_fish_timers(input_file);
    simulate_lantern_fish_population(fish_timers, 256)
}

fn read_lantern_fish_timers<P: AsRef<Path>>(input_file: P) -> Vec<u16> {
    fs::read_to_string(input_file)
        .expect("failed to read data from file")
        .split(',')
        .map(|n| n.parse().expect("failed to parse as number"))
        .collect()
}

fn simulate_lantern_fish_population(fish_timers: Vec<u16>, rounds: usize) -> u64 {
    let mut fish_counts = vec![0; 9];

    for ft in fish_timers {
        fish_counts[ft as usize] += 1;
    }

    let mut new_fish = 0;
    let mut maturing = 0;

    for index in 0..rounds {
        let birth_index = index % 7;

        let matured_this_round = maturing;

        maturing = new_fish;
        new_fish = fish_counts[birth_index];

        fish_counts[birth_index] += matured_this_round;
    }

    fish_counts.iter().sum::<u64>() + maturing + new_fish
}

#[cfg(test)]
mod test {
    use crate::day06::{day_6_1, day_6_2};
    use crate::util::temp_file_with_content;

    const TEST_DATA: &str = "3,4,3,1,2";

    #[test]
    fn test() {
        let file = temp_file_with_content("day_06_01", TEST_DATA);

        assert_eq!(day_6_1(&file), 5934);
        assert_eq!(day_6_2(&file), 26984457539);
    }
}
