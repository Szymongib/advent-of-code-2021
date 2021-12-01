use crate::util::read_lines;
use std::path::Path;

pub fn day_1_1<P: AsRef<Path>>(input_file: P) -> u32 {
    let data: Vec<i32> = read_lines(input_file);

    let mut inc_count: u32 = 0;
    for i in 1..data.len() {
        if data[i] > data[i - 1] {
            inc_count += 1
        }
    }

    inc_count
}

pub fn day_1_2<P: AsRef<Path>>(input_file: P) -> u32 {
    let data: Vec<i32> = read_lines(input_file);

    let mut inc_count: u32 = 0;
    let mut prev_sum: i32 = data.iter().take(3).sum();

    for i in 1..data.len() - 2 {
        let curr_sum = prev_sum - data[i - 1] + data[i + 2];
        if curr_sum > prev_sum {
            inc_count += 1
        }
        prev_sum = curr_sum
    }

    inc_count
}
