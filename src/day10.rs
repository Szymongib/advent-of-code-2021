use crate::util::read_lines_raw;
use std::path::Path;

pub fn day_10_1<P: AsRef<Path>>(input_file: P) -> u64 {
    let chunks_line = read_chunks_lines(input_file);

    chunks_line
        .iter()
        .fold(0, |acc, line| match score_line(line) {
            Score::Invalid(score) => acc + score,
            Score::Autocomplete(_) => acc,
        })
}

pub fn day_10_2<P: AsRef<Path>>(input_file: P) -> u64 {
    let chunks_line = read_chunks_lines(input_file);

    let mut scores: Vec<u64> = chunks_line
        .iter()
        .filter_map(|line| match score_line(line) {
            Score::Invalid(_) => None,
            Score::Autocomplete(score) => Some(score),
        })
        .collect();

    let mid_index = scores.len() / 2;
    let (_, middle, _) = scores.select_nth_unstable(mid_index);

    *middle
}

fn read_chunks_lines<P: AsRef<Path>>(input_file: P) -> Vec<Vec<char>> {
    read_lines_raw(input_file)
        .iter()
        .map(|line| line.chars().collect())
        .collect()
}

enum Score {
    Invalid(u64),
    Autocomplete(u64),
}

fn score_line(chars: &[char]) -> Score {
    let mut stack = vec![];

    for c in chars {
        if is_opening(*c) {
            stack.push(c);
            continue;
        }
        match stack.pop() {
            None => return Score::Invalid(score_closing(*c)),
            Some(open) => {
                if !pair_matching(*open, *c) {
                    return Score::Invalid(score_closing(*c));
                }
            }
        };
    }

    let score = stack
        .iter()
        .rev()
        .fold(0, |acc, c| acc * 5 + score_opening(**c));

    Score::Autocomplete(score)
}

fn is_opening(c: char) -> bool {
    matches!(c, '(' | '[' | '{' | '<')
}

fn pair_matching(open: char, close: char) -> bool {
    matches!(
        (open, close),
        ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>')
    )
}

fn score_closing(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn score_opening(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

#[cfg(test)]
mod test {
    use crate::day10::{day_10_1, day_10_2};
    use crate::util::temp_file_with_content;

    const TEST_DATA: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test() {
        let file = temp_file_with_content("day_10_01", TEST_DATA);

        assert_eq!(day_10_1(&file), 26397);
        assert_eq!(day_10_2(&file), 288957);
    }
}
