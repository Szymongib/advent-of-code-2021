use crate::util::read_lines_raw;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::path::Path;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Explosion {
    digit: Box<Digit>,
    left_rem: u64,
    right_rem: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Reminder {
    Left(u64),
    Right(u64),
}

#[derive(Clone, Debug)]
struct Number {
    lhs: Box<Digit>,
    rhs: Box<Digit>,
}

enum InputToken {
    OpenBracket,
    Number(Number),
    Digit(u64),
}

impl FromStr for Number {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars();

        let mut stack = vec![];

        // TODO: should refactor this part
        for c in chars {
            match c {
                ',' => {}
                '[' => stack.push(InputToken::OpenBracket),
                x if x.is_digit(10) => {
                    stack.push(InputToken::Digit(x.to_digit(10).unwrap() as u64));
                }
                ']' => {
                    let mut digits: Vec<Digit> = vec![];
                    loop {
                        match stack.pop().unwrap() {
                            InputToken::OpenBracket => {
                                // Sanity check that in this case we always have 2 digits to create number
                                assert_eq!(2, digits.len());
                                stack.push(InputToken::Number(Number::new(
                                    digits[1].clone(),
                                    digits[0].clone(),
                                )));
                                break;
                            }
                            InputToken::Number(n) => digits.push(n.into()),
                            InputToken::Digit(val) => digits.push(Digit::Single(val)),
                        }
                    }
                }
                c => unreachable!("unexpected character: {}", c),
            }
        }

        // Expect to have only one element of type Number left.
        assert_eq!(stack.len(), 1);

        match stack.into_iter().next().unwrap() {
            InputToken::Number(num) => Ok(num),
            _ => unreachable!("unexpected token at the end"),
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let fmt = format!("[{},{}]", self.lhs, self.rhs);
        f.write_str(&fmt)
    }
}

impl Number {
    fn new(lhs: Digit, rhs: Digit) -> Number {
        Number {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    fn reduce(&mut self, depth: u8) {
        loop {
            if self.reduce_explosions(depth).is_some() {
                continue;
            }
            if self.reduce_slices() {
                continue;
            }
            break;
        }
    }

    // TODO: probably could further simplify this...
    fn reduce_explosions(&mut self, depth: u8) -> Option<Option<Reminder>> {
        if depth == 3 {
            if let Some(expl) = self.lhs.explode() {
                self.lhs = expl.digit;
                self.rhs.add_left(expl.right_rem);
                return Some(Some(Reminder::Left(expl.left_rem)));
            }
            if let Some(expl) = self.rhs.explode() {
                self.rhs = expl.digit;
                self.lhs.add_right(expl.left_rem);
                return Some(Some(Reminder::Right(expl.right_rem)));
            }
        }

        if let Some(reduction) = self.lhs.reduce_explosions(depth + 1) {
            return match reduction {
                Some(Reminder::Right(num)) => {
                    self.rhs.add_left(num);
                    Some(None) // Indicates no reminder left, but the operation was performed
                }
                r => Some(r),
            };
        }

        if let Some(reduction) = self.rhs.reduce_explosions(depth + 1) {
            return match reduction {
                Some(Reminder::Left(num)) => {
                    self.lhs.add_right(num);
                    Some(None) // Indicates no reminder left, but the operation was performed
                }
                r => Some(r),
            };
        }
        None
    }

    fn reduce_slices(&mut self) -> bool {
        // Stop on first slice, so that we can check for explosions immediately after.
        self.lhs.slice() || self.rhs.slice()
    }

    fn magnitude(&self) -> u64 {
        self.lhs.magnitude() * 3 + self.rhs.magnitude() * 2
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        Number::new(self.into(), rhs.into())
    }
}

impl From<Number> for Digit {
    fn from(num: Number) -> Self {
        Digit::Number(num)
    }
}

#[derive(Clone, Debug)]
enum Digit {
    Single(u64),
    Number(Number),
}

impl Display for Digit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Digit::Single(num) => f.write_str(&num.to_string()),
            Digit::Number(num) => f.write_str(&format!("{}", num)),
        }
    }
}

impl Digit {
    fn reduce_explosions(&mut self, depth: u8) -> Option<Option<Reminder>> {
        match self {
            Digit::Single(_) => None,
            Digit::Number(num) => num.reduce_explosions(depth),
        }
    }

    fn slice(&mut self) -> bool {
        match self {
            Digit::Single(x) if *x >= 10 => {
                let split: f64 = (*x as f64) / 2.;
                let l = split.floor() as u64;
                let r = split.ceil() as u64;

                *self = Digit::Number(Number::new(Digit::Single(l), Digit::Single(r)));
                true
            }
            Digit::Single(_) => false,
            Digit::Number(num) => num.reduce_slices(),
        }
    }

    fn explode(&self) -> Option<Explosion> {
        match self {
            Digit::Single(_) => None,
            Digit::Number(num) => match (*num.lhs.clone(), *num.rhs.clone()) {
                (Digit::Single(lhs), Digit::Single(rhs)) => Some(Explosion {
                    digit: Box::new(Digit::Single(0)),
                    left_rem: lhs,
                    right_rem: rhs,
                }),
                _ => unreachable!("unexpected explosion on complex number"),
            },
        }
    }

    fn add_right(&mut self, value: u64) {
        match self {
            Digit::Single(v) => *v += value,
            Digit::Number(num) => {
                num.rhs.add_right(value);
            }
        }
    }

    fn add_left(&mut self, value: u64) {
        match self {
            Digit::Single(v) => *v += value,
            Digit::Number(num) => {
                num.lhs.add_left(value);
            }
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            Digit::Single(num) => *num,
            Digit::Number(num) => num.magnitude(),
        }
    }
}

pub fn day_18_1<P: AsRef<Path>>(input_file: P) -> u64 {
    let numbers = parse_numbers(input_file);

    let mut number = numbers[0].clone();

    for num in numbers.into_iter().skip(1) {
        number = number.add(num);
        number.reduce(0);
    }

    number.magnitude()
}

pub fn day_18_2<P: AsRef<Path>>(input_file: P) -> u64 {
    let numbers = parse_numbers(input_file);

    let mut max_magnitude = 0;

    for i in 0..numbers.len() {
        for k in 0..numbers.len() {
            if i == k {
                continue;
            }
            let mut number = numbers[i].clone().add(numbers[k].clone());
            number.reduce(0);
            let mag = number.magnitude();
            max_magnitude = max_magnitude.max(mag);
        }
    }

    max_magnitude
}

fn parse_numbers<P: AsRef<Path>>(input_file: P) -> Vec<Number> {
    read_lines_raw(input_file)
        .into_iter()
        .map(|line| Number::from_str(&line).unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use crate::day18::{day_18_1, day_18_2};
    use crate::util::temp_file_with_content;

    const TEST_DATA: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn test() {
        let file = temp_file_with_content("day_18", TEST_DATA);

        assert_eq!(day_18_1(&file), 4140);
        assert_eq!(day_18_2(&file), 3993);
    }
}
