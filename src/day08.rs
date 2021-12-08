use crate::util::{read_lines, read_lines_raw};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::path::Path;
use std::str::FromStr;

pub fn day_8_1<P: AsRef<Path>>(input_file: P) -> usize {
    let digits: Vec<usize> = read_lines_raw(input_file)
        .iter()
        .map(|line| {
            let (_, p2) = line.split_once('|').expect("expected 2 parts");
            p2.trim()
                .split(' ')
                .map(|enc_dig| enc_dig.len())
                .collect::<Vec<usize>>()
        })
        .flatten()
        .collect();

    let mut counts: [usize; 8] = [0; 8];

    for d in &digits {
        counts[*d] += 1;
    }

    counts[2] + counts[3] + counts[4] + counts[7]
}

// This solution is super ugly and I did not find a reasonable abstraction
// in limited time I spent for solving this problem. I am sure it can be greatly improved.
// Anyway, this works and I have some other stuff to do so hopefully
// ancient gods of coding will not send their punishment on me...

#[derive(PartialEq, Debug, Clone, Copy, Hash, Eq)]
enum SignalPosition {
    // Top,
    // TopLeft,
    TopRight,
    Middle,
    BottomLeft,
    // BottomRight,
    // Bottom,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DigitRepresentation(String);

impl FromStr for DigitRepresentation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();

        Ok(DigitRepresentation(String::from_iter(chars)))
    }
}

impl DigitRepresentation {
    fn to_digit(&self) -> Option<usize> {
        match self.0.len() {
            2 => Some(1),
            4 => Some(4),
            3 => Some(7),
            7 => Some(8),
            _ => None,
        }
    }

    fn diff(&self, other: &DigitRepresentation) -> Vec<char> {
        self.0
            .clone()
            .chars()
            .into_iter()
            .filter(|ele| !other.0.contains(&ele.to_string()))
            .collect()
    }
    fn common(&self, other: &DigitRepresentation) -> Vec<char> {
        self.0
            .clone()
            .chars()
            .into_iter()
            .filter(|ele| other.0.contains(&ele.to_string()))
            .collect()
    }
}

struct DisplayData {
    signals: Vec<DigitRepresentation>,
    digit_rep: Vec<Option<DigitRepresentation>>,
    positions: HashMap<SignalPosition, char>,
}

impl DisplayData {
    fn new(signals: Vec<DigitRepresentation>) -> DisplayData {
        DisplayData {
            signals,
            digit_rep: vec![None; 10],
            positions: HashMap::new(),
        }
    }

    fn calc_value(&self) -> usize {
        let elems = &self.signals[self.signals.len() - 4..];

        let mut value = 0;
        for elem in elems.iter() {
            value *= 10;
            let digit = self
                .digit_rep
                .iter()
                .position(|e| elem == e.as_ref().unwrap())
                .expect("expected value");
            value += digit;
        }
        value
    }

    fn determine_nine(
        &self,
        six_elem_signals: &[DigitRepresentation],
    ) -> Option<DigitRepresentation> {
        let fourth = self.digit_rep[4].clone()?;

        // Find 9 by finding 6 len digit that has 4 common signals with 4
        six_elem_signals
            .iter()
            .find(|e| e.common(&fourth).len() == 4)
            .cloned()
    }

    fn determine_middle_and_bottom_left(&mut self, zero_six: &[DigitRepresentation]) {
        // 8 and 6 different signal will be TopRight
        // 8 and 0 different signal will be Middle
        let diffs: Vec<Vec<char>> = vec![
            self.digit_rep[8].as_ref().unwrap().diff(&zero_six[0]),
            self.digit_rep[8].as_ref().unwrap().diff(&zero_six[1]),
        ];

        // We determine which one is 0 and which one is 6.
        let (num_zero_index, num_six_index) = if !self.digit_rep[1]
            .as_ref()
            .unwrap()
            .0
            .contains(&diffs[0][0].to_string())
        {
            (0, 1)
        } else {
            (1, 0)
        };

        self.positions
            .insert(SignalPosition::Middle, diffs[num_zero_index][0]);
        self.digit_rep[0] = Some(zero_six[num_zero_index].clone());
        self.positions
            .insert(SignalPosition::TopRight, diffs[num_six_index][0]);
        self.digit_rep[6] = Some(zero_six[num_six_index].clone());

        // We now have 6 so we can find the signal that does not belong to 9 and therefore get BottomLeft.
        let diff = self.digit_rep[6]
            .as_ref()
            .unwrap()
            .diff(self.digit_rep[9].as_ref().unwrap());
        self.positions.insert(SignalPosition::BottomLeft, diff[0]);
    }

    fn determine_five_len(&mut self, mut five_len: Vec<DigitRepresentation>) {
        // We get 2 by finding the only 5 signals digit that does contain BottomLeft signal.
        let two = five_len
            .iter()
            .find(|e| {
                e.0.contains(
                    &self
                        .positions
                        .get(&SignalPosition::BottomLeft)
                        .unwrap()
                        .to_string(),
                )
            })
            .unwrap();

        self.digit_rep[2] = Some(two.clone());

        // We remove 2 from 5 signal digits.
        five_len = five_len
            .into_iter()
            .filter(|e| {
                !e.0.contains(
                    &self
                        .positions
                        .get(&SignalPosition::BottomLeft)
                        .unwrap()
                        .to_string(),
                )
            })
            .collect();

        // Of two remaining we check which one contains TopRight signal - this will be 3, the other 5.
        let (five_index, three_index) = if !five_len[0].0.contains(
            &self
                .positions
                .get(&SignalPosition::TopRight)
                .unwrap()
                .to_string(),
        ) {
            (0, 1)
        } else {
            (1, 0)
        };

        self.digit_rep[5] = Some(five_len[five_index].clone());
        self.digit_rep[3] = Some(five_len[three_index].clone());
    }

    fn decode(&mut self) {
        let signals = self.signals.clone();

        // Those will be 0, 6 and 9
        let mut six_len_ele_signals: Vec<DigitRepresentation> = vec![];

        // Those will be 2, 3 and 5
        let mut five_ele_signals: Vec<DigitRepresentation> = vec![];

        for sig in &signals {
            // We determine 1, 4, 7 and 8 right away
            if let Some(dig) = sig.to_digit() {
                self.digit_rep[dig] = Some(sig.clone());
            } else if sig.0.len() == 6 && !six_len_ele_signals.contains(sig) {
                six_len_ele_signals.push(sig.clone());
            } else if sig.0.len() == 5 && !five_ele_signals.contains(sig) {
                five_ele_signals.push(sig.clone());
            }
        }

        self.digit_rep[9] = self.determine_nine(&six_len_ele_signals);
        six_len_ele_signals = six_len_ele_signals
            .into_iter()
            .filter(|e| e != self.digit_rep[9].as_ref().unwrap())
            .collect();

        // This also determines 0 and 6
        self.determine_middle_and_bottom_left(&six_len_ele_signals);

        self.determine_five_len(five_ele_signals);
    }
}

impl FromStr for DisplayData {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let signals: Vec<&str> = s
            .split(' ')
            .filter(|e| !e.is_empty() && e != &" " && e != &"|")
            .collect();

        let signals = signals
            .into_iter()
            .map(|s| DigitRepresentation::from_str(s).expect("failed to parse signal data"))
            .collect();

        Ok(DisplayData::new(signals))
    }
}

pub fn day_8_2<P: AsRef<Path>>(input_file: P) -> usize {
    let mut display_data: Vec<DisplayData> = read_lines(input_file);

    let mut sum = 0;
    for dd in display_data.iter_mut() {
        dd.decode();
        sum += dd.calc_value();
    }

    sum
}

#[cfg(test)]
mod test {
    use crate::day08::{day_8_1, day_8_2};
    use crate::util::temp_file_with_content;

    const TEST_DATA: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test() {
        let file = temp_file_with_content("day_08_01", TEST_DATA);

        assert_eq!(day_8_1(&file), 26);
        assert_eq!(day_8_2(&file), 61229);
    }
}
