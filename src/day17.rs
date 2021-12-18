use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Range(i32, i32);

impl Range {
    fn new(a: i32, b: i32) -> Range {
        Range {
            0: a.min(b),
            1: a.max(b),
        }
    }
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s
            .split_once("..")
            .map(|(min, max)| (min.parse().unwrap(), max.parse().unwrap()))
            .unwrap();
        Ok(Range::new(val.0, val.1))
    }
}

pub fn day_17_1<P: AsRef<Path>>(input_file: P) -> usize {
    let (_, y) = read_ranges(input_file);
    let max_y_vel = y.0.abs() as usize - 1;

    (max_y_vel) * (max_y_vel + 1) / 2
}

pub fn day_17_2<P: AsRef<Path>>(input_file: P) -> usize {
    let (x, y) = read_ranges(input_file);

    let max_y_vel = y.0.abs() - 1;
    let min_y_vel = y.0;

    // This is max number of steps we need to consider in regards to x values
    // that will stop inside the area as any other y will just miss it.
    let max_steps_for_x = (2 * max_y_vel + 2) as usize;

    let max_x = x.1;
    let min_x = calc_x_min(x.0);

    // Maps number of steps to particular x values.
    let mut steps_to_x_val: HashMap<usize, Vec<i32>> = HashMap::new();
    for x_val in min_x..max_x + 1 {
        get_xes_at_steps(x_val, &mut steps_to_x_val, x, max_steps_for_x);
    }

    let mut hash_set: HashSet<(i32, i32)> = HashSet::new();
    for y_val in min_y_vel..max_y_vel + 1 {
        calc_points(y_val, &steps_to_x_val, y, &mut hash_set);
    }

    hash_set.len()
}

fn read_ranges<P: AsRef<Path>>(input_file: P) -> (Range, Range) {
    let data = fs::read_to_string(input_file).expect("failed to read input");
    let (x_range, y_range) = data
        .strip_prefix("target area: ")
        .unwrap()
        .split_once(", ")
        .unwrap();

    let x = Range::from_str(x_range.strip_prefix("x=").unwrap()).unwrap();
    let y = Range::from_str(y_range.strip_prefix("y=").unwrap()).unwrap();

    (x, y)
}

fn get_xes_at_steps(
    init_vel: i32,
    steps_count: &mut HashMap<usize, Vec<i32>>,
    x_range: Range,
    max_steps: usize,
) {
    let mut vel = init_vel;
    let mut x_pos = 0;
    let mut steps = 0;

    while x_pos <= x_range.1 && steps <= max_steps {
        if x_pos >= x_range.0 {
            match steps_count.get_mut(&steps) {
                None => {
                    steps_count.insert(steps, vec![init_vel]);
                }
                Some(curr) => curr.push(init_vel),
            }
        }
        x_pos += vel;
        if vel > 0 {
            vel -= 1;
        }
        steps += 1;
    }
}

fn calc_points(
    init_vel: i32,
    x_steps: &HashMap<usize, Vec<i32>>,
    y_range: Range,
    hashs: &mut HashSet<(i32, i32)>,
) {
    let mut vel = init_vel;
    let mut y_pos = 0;
    let mut steps = 0;

    while y_pos >= y_range.0 {
        if y_pos <= y_range.1 {
            match x_steps.get(&steps) {
                None => {}
                Some(curr) => {
                    for x in curr {
                        hashs.insert((*x, init_vel));
                    }
                }
            }
        }
        y_pos += vel;
        vel -= 1;
        steps += 1;
    }
}

fn calc_x_min(min_x: i32) -> i32 {
    let mut val = 1;

    while val * val + val < min_x * 2 {
        val += 1;
    }
    val
}

#[cfg(test)]
mod test {
    use crate::day17::{day_17_1, day_17_2};
    use crate::util::temp_file_with_content;

    #[test]
    fn test() {
        let file = temp_file_with_content("day_17", "target area: x=20..30, y=-10..-5");

        assert_eq!(day_17_1(&file), 45);
        assert_eq!(day_17_2(&file), 112);
    }
}
