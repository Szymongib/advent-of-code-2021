use std::fmt::Display;
use std::path::Path;
use std::{env, process};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
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
    use std::fmt::Display;
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
    }
}
