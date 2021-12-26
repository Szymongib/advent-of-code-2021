use std::fmt::Display;
use std::path::Path;
use std::{env, process};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod util;

pub fn run_task<T, O, P>(func: T, day: u16, task: u16, input_file: P)
where
    T: Fn(P) -> O,
    O: Display,
    P: AsRef<Path>,
{
    println!("Running Day {} task {}", day, task);

    let out = func(input_file);

    println!("Output: {}", out);
}

fn main() {
    println!("Running Advent of code 2021!");

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Invalid arguments. Provide [DAY] [TASK] as an arguments.");
        process::exit(1);
    }

    let args: [&str; 2] = [&args[1], &args[2]];

    match args {
        ["1", "1"] => run_task(day01::day_1_1, 1, 1, "inputs/1.1.txt"),
        ["1", "2"] => run_task(day01::day_1_2, 1, 2, "inputs/1.1.txt"),
        ["2", "1"] => run_task(day02::day_2_1, 2, 1, "inputs/2.1.txt"),
        ["2", "2"] => run_task(day02::day_2_2, 2, 2, "inputs/2.1.txt"),
        ["3", "1"] => run_task(day03::day_3_1, 3, 1, "inputs/3.1.txt"),
        ["3", "2"] => run_task(day03::day_3_2, 3, 2, "inputs/3.1.txt"),
        ["4", "1"] => run_task(day04::day_4_1, 4, 1, "inputs/4.1.txt"),
        ["4", "2"] => run_task(day04::day_4_2, 4, 2, "inputs/4.1.txt"),
        ["5", "1"] => run_task(day05::day_5_1, 5, 1, "inputs/5.1.txt"),
        ["5", "2"] => run_task(day05::day_5_2, 5, 2, "inputs/5.1.txt"),
        ["6", "1"] => run_task(day06::day_6_1, 6, 1, "inputs/6.1.txt"),
        ["6", "2"] => run_task(day06::day_6_2, 6, 2, "inputs/6.1.txt"),
        ["7", "1"] => run_task(day07::day_7_1, 7, 1, "inputs/7.1.txt"),
        ["7", "2"] => run_task(day07::day_7_2, 7, 2, "inputs/7.1.txt"),
        ["8", "1"] => run_task(day08::day_8_1, 8, 1, "inputs/8.txt"),
        ["8", "2"] => run_task(day08::day_8_2, 8, 2, "inputs/8.txt"),
        ["9", "1"] => run_task(day09::day_9_1, 9, 1, "inputs/9.txt"),
        ["9", "2"] => run_task(day09::day_9_2, 9, 2, "inputs/9.txt"),
        ["10", "1"] => run_task(day10::day_10_1, 10, 1, "inputs/10.txt"),
        ["10", "2"] => run_task(day10::day_10_2, 10, 2, "inputs/10.txt"),
        ["11", "1"] => run_task(day11::day_11_1, 11, 1, "inputs/11.txt"),
        ["11", "2"] => run_task(day11::day_11_2, 11, 2, "inputs/11.txt"),
        ["12", "1"] => run_task(day12::day_12_1, 12, 1, "inputs/12.txt"),
        ["12", "2"] => run_task(day12::day_12_2, 12, 2, "inputs/12.txt"),
        ["13", "1"] => run_task(day13::day_13_1, 13, 1, "inputs/13.txt"),
        ["13", "2"] => run_task(day13::day_13_2, 13, 2, "inputs/13.txt"),
        ["14", "1"] => run_task(day14::day_14_1, 14, 1, "inputs/14.txt"),
        ["14", "2"] => run_task(day14::day_14_2, 14, 2, "inputs/14.txt"),
        ["15", "1"] => run_task(day15::day_15_1, 15, 1, "inputs/15.txt"),
        ["15", "2"] => run_task(day15::day_15_2, 15, 2, "inputs/15.txt"),
        ["16", "1"] => run_task(day16::day_16_1, 16, 1, "inputs/16.txt"),
        ["16", "2"] => run_task(day16::day_16_2, 16, 2, "inputs/16.txt"),
        ["17", "1"] => run_task(day17::day_17_1, 17, 1, "inputs/17.txt"),
        ["17", "2"] => run_task(day17::day_17_2, 17, 2, "inputs/17.txt"),
        ["18", "1"] => run_task(day18::day_18_1, 18, 1, "inputs/18.txt"),
        ["18", "2"] => run_task(day18::day_18_2, 18, 2, "inputs/18.txt"),
        ["19", "1"] => run_task(day19::day_19_1, 19, 1, "inputs/19.txt"),
        ["19", "2"] => run_task(day19::day_19_2, 19, 2, "inputs/19.txt"),
        ["20", "1"] => run_task(day20::day_20_1, 20, 1, "inputs/20.txt"),
        ["20", "2"] => run_task(day20::day_20_2, 20, 2, "inputs/20.txt"),
        ["21", "1"] => run_task(day21::day_21_1, 21, 1, "inputs/21.txt"),
        ["21", "2"] => run_task(day21::day_21_2, 21, 2, "inputs/21.txt"),
        ["22", "1"] => run_task(day22::day_22_1, 22, 1, "inputs/22.txt"),
        ["22", "2"] => run_task(day22::day_22_2, 22, 2, "inputs/22.txt"),
        ["23", "1"] => run_task(day23::day_23_1, 23, 1, "inputs/23.txt"),
        ["23", "2"] => run_task(day23::day_23_2, 23, 2, "inputs/23.txt"),
        ["24", "1"] => run_task(day24::day_24_1, 24, 1, "inputs/24.txt"),
        ["24", "2"] => run_task(day24::day_24_2, 24, 2, "inputs/24.txt"),
        ["25", "1"] => run_task(day25::day_25_1, 25, 1, "inputs/25.txt"),
        ["25", "2"] => run_task(day25::day_25_2, 25, 2, "inputs/25.txt"),
        [day, task] => {
            println!("Invalid arguments, day: {}, task: {}", day, task);
            process::exit(1)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day01::{day_1_1, day_1_2};
    use crate::day02::{day_2_1, day_2_2};
    use crate::day03::{day_3_1, day_3_2};
    use crate::day04::{day_4_1, day_4_2};
    use crate::day05::{day_5_1, day_5_2};
    use crate::day06::{day_6_1, day_6_2};
    use crate::day07::{day_7_1, day_7_2};
    use crate::day08::{day_8_1, day_8_2};
    use crate::day09::{day_9_1, day_9_2};
    use crate::day10::{day_10_1, day_10_2};
    use crate::day11::{day_11_1, day_11_2};
    use crate::day12::{day_12_1, day_12_2};
    use crate::day13::{day_13_1, day_13_2};
    use crate::day14::{day_14_1, day_14_2};
    use crate::day15::{day_15_1, day_15_2};
    use crate::day16::{day_16_1, day_16_2};
    use crate::day17::{day_17_1, day_17_2};
    use crate::day18::{day_18_1, day_18_2};
    use crate::day19::{day_19_1, day_19_2};
    use crate::day20::{day_20_1, day_20_2};
    use crate::day21::{day_21_1, day_21_2};
    use crate::day22::{day_22_1, day_22_2};
    use crate::day23::{day_23_1, day_23_2};
    use crate::day24::{day_24_1, day_24_2};
    use crate::day25::day_25_1;
    use std::fs;
    use std::path::Path;
    use std::str::FromStr;

    fn read_output<P: AsRef<Path>, T: FromStr>(out_file: P) -> T
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        fs::read_to_string(out_file)
            .expect("failed to read out file")
            .parse()
            .expect("failed to parse output")
    }

    fn in_path(file: &str) -> String {
        format!("inputs/{}", file)
    }

    fn out_path(file: &str) -> String {
        format!("outputs/{}", file)
    }

    #[test]
    fn test_with_inputs() {
        assert_eq!(
            day_1_1(in_path("1.1.txt")),
            read_output(out_path("1.1.txt"))
        );
        assert_eq!(
            day_1_2(in_path("1.1.txt")),
            read_output(out_path("1.2.txt"))
        );
        assert_eq!(
            day_2_1(in_path("2.1.txt")),
            read_output(out_path("2.1.txt"))
        );
        assert_eq!(
            day_2_2(in_path("2.1.txt")),
            read_output(out_path("2.2.txt"))
        );
        assert_eq!(
            day_3_1(in_path("3.1.txt")),
            read_output(out_path("3.1.txt"))
        );
        assert_eq!(
            day_3_2(in_path("3.1.txt")),
            read_output(out_path("3.2.txt"))
        );
        assert_eq!(
            day_4_1(in_path("4.1.txt")),
            read_output(out_path("4.1.txt"))
        );
        assert_eq!(
            day_4_2(in_path("4.1.txt")),
            read_output(out_path("4.2.txt"))
        );
        assert_eq!(
            day_5_1(in_path("5.1.txt")),
            read_output(out_path("5.1.txt"))
        );
        assert_eq!(
            day_5_2(in_path("5.1.txt")),
            read_output(out_path("5.2.txt"))
        );
        assert_eq!(
            day_6_1(in_path("6.1.txt")),
            read_output(out_path("6.1.txt"))
        );
        assert_eq!(
            day_6_2(in_path("6.1.txt")),
            read_output(out_path("6.2.txt"))
        );
        assert_eq!(
            day_7_1(in_path("7.1.txt")),
            read_output(out_path("7.1.txt"))
        );
        assert_eq!(
            day_7_2(in_path("7.1.txt")),
            read_output(out_path("7.2.txt"))
        );
        assert_eq!(day_8_1(in_path("8.txt")), read_output(out_path("8.1.txt")));
        assert_eq!(day_8_2(in_path("8.txt")), read_output(out_path("8.2.txt")));
        assert_eq!(day_9_1(in_path("9.txt")), read_output(out_path("9.1.txt")));
        assert_eq!(day_9_2(in_path("9.txt")), read_output(out_path("9.2.txt")));
        assert_eq!(
            day_10_1(in_path("10.txt")),
            read_output(out_path("10.1.txt"))
        );
        assert_eq!(
            day_10_2(in_path("10.txt")),
            read_output(out_path("10.2.txt"))
        );
        assert_eq!(
            day_11_1(in_path("11.txt")),
            read_output(out_path("11.1.txt"))
        );
        assert_eq!(
            day_11_2(in_path("11.txt")),
            read_output(out_path("11.2.txt"))
        );
        assert_eq!(
            day_12_1(in_path("12.txt")),
            read_output(out_path("12.1.txt"))
        );
        assert_eq!(
            day_12_2(in_path("12.txt")),
            read_output(out_path("12.2.txt"))
        );
        assert_eq!(
            day_13_1(in_path("13.txt")),
            read_output(out_path("13.1.txt"))
        );
        assert_eq!(
            day_13_2(in_path("13.txt")),
            read_output::<_, String>(out_path("13.2.txt"))
        );
        assert_eq!(
            day_14_1(in_path("14.txt")),
            read_output(out_path("14.1.txt"))
        );
        assert_eq!(
            day_14_2(in_path("14.txt")),
            read_output(out_path("14.2.txt"))
        );
        assert_eq!(
            day_15_1(in_path("15.txt")),
            read_output(out_path("15.1.txt"))
        );
        assert_eq!(
            day_15_2(in_path("15.txt")),
            read_output(out_path("15.2.txt"))
        );
        assert_eq!(
            day_16_1(in_path("16.txt")),
            read_output(out_path("16.1.txt"))
        );
        assert_eq!(
            day_16_2(in_path("16.txt")),
            read_output(out_path("16.2.txt"))
        );
        assert_eq!(
            day_17_1(in_path("17.txt")),
            read_output(out_path("17.1.txt"))
        );
        assert_eq!(
            day_17_2(in_path("17.txt")),
            read_output(out_path("17.2.txt"))
        );
        assert_eq!(
            day_18_1(in_path("18.txt")),
            read_output(out_path("18.1.txt"))
        );
        assert_eq!(
            day_18_2(in_path("18.txt")),
            read_output(out_path("18.2.txt"))
        );
        // Those take too long when not running --release
        #[cfg(not(debug_assertions))]
        assert_eq!(
            day_19_1(in_path("19.txt")),
            read_output(out_path("19.1.txt"))
        );
        #[cfg(not(debug_assertions))]
        assert_eq!(
            day_19_2(in_path("19.txt")),
            read_output(out_path("19.2.txt"))
        );
        assert_eq!(
            day_20_1(in_path("20.txt")),
            read_output(out_path("20.1.txt"))
        );
        assert_eq!(
            day_20_2(in_path("20.txt")),
            read_output(out_path("20.2.txt"))
        );
        assert_eq!(
            day_21_1(in_path("21.txt")),
            read_output(out_path("21.1.txt"))
        );
        assert_eq!(
            day_21_2(in_path("21.txt")),
            read_output(out_path("21.2.txt"))
        );
        assert_eq!(
            day_22_1(in_path("22.txt")),
            read_output(out_path("22.1.txt"))
        );
        assert_eq!(
            day_22_2(in_path("22.txt")),
            read_output(out_path("22.2.txt"))
        );
        assert_eq!(
            day_23_1(in_path("23.txt")),
            read_output(out_path("23.1.txt"))
        );
        assert_eq!(
            day_23_2(in_path("23.txt")),
            read_output(out_path("23.2.txt"))
        );
        assert_eq!(
            day_24_1(in_path("24.txt")),
            read_output(out_path("24.1.txt"))
        );
        assert_eq!(
            day_24_2(in_path("24.txt")),
            read_output(out_path("24.2.txt"))
        );
        assert_eq!(
            day_25_1(in_path("25.txt")),
            read_output(out_path("25.1.txt"))
        );
    }
}
