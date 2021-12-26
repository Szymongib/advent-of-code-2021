use std::fs;
use std::path::Path;

// This is hacky solution based on some tips from Reddit.
// Apparently all inputs had similar characteristics and I was to tired...

pub fn day_24_1<P: AsRef<Path>>(input_file: P) -> usize {
    let data = fs::read_to_string(input_file).unwrap();
    let (_, max) = find_min_max(&data);
    max
}

pub fn day_24_2<P: AsRef<Path>>(input_file: P) -> usize {
    let data = fs::read_to_string(input_file).unwrap();
    let (min, _) = find_min_max(&data);
    min
}

fn find_min_max(input: &str) -> (usize, usize) {
    let mut max_digits = [0; 14];
    let mut min_digits = [0; 14];

    let mut stack = vec![];

    let inst = input
        .split("inp w\n")
        .skip(1)
        .map(|s| s.lines().collect::<Vec<_>>());

    for (idx, instructions) in inst.enumerate() {
        let z_division = instructions[3]
            .strip_prefix("div z ")
            .unwrap()
            .parse::<i8>()
            .unwrap();

        if z_division == 1 {
            let f = instructions[14]
                .strip_prefix("add y ")
                .unwrap()
                .parse::<i8>()
                .unwrap();
            stack.push((idx, f));
        } else if z_division == 26 {
            let (mut prev_idx, x) = stack.pop().unwrap();
            let mut diff = x + instructions[4]
                .strip_prefix("add x ")
                .unwrap()
                .parse::<i8>()
                .unwrap();

            let mut idx = idx;
            if diff < 0 {
                core::mem::swap(&mut idx, &mut prev_idx);
                diff = -diff;
            }

            max_digits[idx] = 9;
            max_digits[prev_idx] = 9 - diff as u8;
            min_digits[idx] = 1 + diff as u8;
            min_digits[prev_idx] = 1;
        }
    }
    let max = to_num(&max_digits);
    let min = to_num(&min_digits);
    (min, max)
}

fn to_num(digits: &[u8]) -> usize {
    digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &x)| x as usize * 10usize.pow(i as u32))
        .sum::<usize>()
}
