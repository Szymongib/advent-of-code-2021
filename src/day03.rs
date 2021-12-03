use crate::util::read_lines;
use std::path::Path;

pub fn day_3_1<P: AsRef<Path>>(input_file: P) -> usize {
    let data: Vec<String> = read_lines(input_file);

    let len = data.len();
    let word_len = data[0].len();

    let mut ones = vec![0; word_len];

    for s in data {
        for (i, c) in s.chars().enumerate() {
            if c == '1' {
                ones[i] += 1;
            }
        }
    }

    let mut gamma: usize = 0;
    let mut epsilon: usize = 0;

    for (i, c) in ones.iter().enumerate() {
        if *c as usize > len / 2 {
            gamma += 1 << (word_len - i - 1)
        } else {
            epsilon += 1 << (word_len - i - 1)
        }
    }

    gamma * epsilon
}

pub fn day_3_2<P: AsRef<Path>>(input_file: P) -> usize {
    let data: Vec<String> = read_lines(input_file);

    let word_len = data[0].len();

    let nums: Vec<usize> = data
        .iter()
        .map(|v| usize::from_str_radix(v, 2).expect("failed to parse from binary"))
        .collect();

    let mut ox_gen_rating = filter_by_bit_criteria(nums.clone(), word_len, 1);
    let mut co2_scrub_rating = filter_by_bit_criteria(nums.clone(), word_len, 0);

    ox_gen_rating * co2_scrub_rating
}

fn filter_by_bit_criteria(mut nums: Vec<usize>, word_len: usize, default: usize) -> usize {
    let mut shift = word_len;
    while nums.len() != 1 {
        shift -= 1;
        let filter_val = if count_ones_at_pos(&nums, shift) >= (nums.len() + 1) / 2 {
            default
        } else {
            default ^ 1
        };

        nums = nums
            .into_iter()
            .filter(|n| (*n >> shift) % 2 == filter_val)
            .collect();
    }

    return nums[0];
}

fn count_ones_at_pos(nums: &[usize], shift: usize) -> usize {
    nums.iter().filter(|n| ((**n) >> shift) % 2 == 1).count()
}
