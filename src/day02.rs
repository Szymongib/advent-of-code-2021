use crate::util::read_lines;
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;

enum Direction {
    Up,
    Down,
    Forward,
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            other => Err(ParseError::UnexpectedDirectionError(other.to_owned())),
        }
    }
}

struct Instruction {
    dir: Direction,
    value: u32,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        if parts.len() != 2 {
            return Err(ParseError::InvalidSplitError(format!(
                "expected 2 parts, got {}",
                parts.len()
            )));
        }

        let instruction = Instruction {
            dir: Direction::from_str(parts[0])?,
            value: parts[1].parse().map_err(|e| ParseError::ParseIntError(e))?,
        };
        Ok(instruction)
    }
}

#[derive(Debug)]
enum ParseError {
    InvalidSplitError(String),
    UnexpectedDirectionError(String),
    ParseIntError(std::num::ParseIntError),
}

pub fn day_2_1<P: AsRef<Path>>(input_file: P) -> u32 {
    let instructions: Vec<Instruction> = read_lines(input_file);

    let mut depth: u32 = 0;
    let mut pos: u32 = 0;

    for inst in instructions {
        match inst {
            Instruction {
                dir: Direction::Up,
                value: v,
            } => depth -= v,
            Instruction {
                dir: Direction::Down,
                value: v,
            } => depth += v,
            Instruction {
                dir: Direction::Forward,
                value: v,
            } => pos += v,
        };
    }

    depth * pos
}

pub fn day_2_2<P: AsRef<Path>>(input_file: P) -> u32 {
    let instructions: Vec<Instruction> = read_lines(input_file);

    let mut depth: u32 = 0;
    let mut pos: u32 = 0;
    let mut aim: i32 = 0;

    for inst in instructions {
        match inst {
            Instruction {
                dir: Direction::Up,
                value: v,
            } => aim -= v as i32,
            Instruction {
                dir: Direction::Down,
                value: v,
            } => aim += v as i32,
            Instruction {
                dir: Direction::Forward,
                value: v,
            } => {
                pos += v;
                depth += (aim * v as i32) as u32;
            }
        };
    }

    depth * pos
}
