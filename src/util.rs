use std::env::temp_dir;
use std::path::{Path, PathBuf};
use std::{fs, str::FromStr};

pub fn read_lines_raw<P: AsRef<Path>>(file_name: P) -> Vec<String> {
    let data = fs::read_to_string(file_name).expect("read data from file");
    let data: Vec<String> = data.split('\n').map(|s| s.to_string()).collect();

    data
}

pub fn read_lines<T: FromStr, P: AsRef<Path>>(file_name: P) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let data = fs::read_to_string(file_name).expect("read data from file");

    data.split('\n')
        .into_iter()
        .map(|s| s.parse().expect("cannot convert from &str to T"))
        .collect()
}

pub fn temp_file_with_content(name: &str, content: &str) -> PathBuf {
    let mut full_path = temp_dir();
    full_path.push(name);

    fs::write(&full_path, content).expect("failed to write temp file");
    full_path
}
