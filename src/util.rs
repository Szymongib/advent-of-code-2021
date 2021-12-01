use std::path::Path;
use std::{fs, str::FromStr};

pub fn read_lines_raw<P: AsRef<Path>>(file_name: P) -> Vec<String> {
    let data = fs::read_to_string(file_name).expect("read data from file");

    let data: Vec<String> = data.split("\n").map(|s| s.to_string()).collect();

    println!("Data len: {}", data.len());

    data
}

pub fn read_lines<T: FromStr, P: AsRef<Path>>(file_name: P) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let data = fs::read_to_string(file_name).expect("read data from file");

    data.split("\n")
        .into_iter()
        .map(|s| s.parse().expect("cannot convert from &str to T"))
        .collect()
}
