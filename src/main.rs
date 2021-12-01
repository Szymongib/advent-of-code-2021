use std::fmt::Display;
use std::path::Path;
use std::{env, process};

mod day01;
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
        [day, task] => {
            println!("Invalid arguments, day: {}, task: {}", day, task);
            process::exit(1)
        }
    }
}
