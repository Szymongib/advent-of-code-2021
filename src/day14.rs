use std::collections::HashMap;
use std::path::Path;

pub fn day_14_1<P: AsRef<Path>>(input_file: P) -> u64 {
    find_most_least_frequent_diff(input_file, 10)
}

pub fn day_14_2<P: AsRef<Path>>(input_file: P) -> u64 {
    find_most_least_frequent_diff(input_file, 40)
}

fn find_most_least_frequent_diff<P: AsRef<Path>>(input_file: P, rounds: usize) -> u64 {
    let (template, inserts) = read_data(input_file);

    let mut pair_map = HashMap::new();

    // Each pair after insert result in 2 pairs.
    // As there is finite number of pairs, create mapping for each.
    for i in 0..template.len() - 1 {
        make_pair_map(&template[i..i + 2], &inserts, &mut pair_map);
    }

    let mut counts: [u64; 26] = [0; 26];

    // Mem maps round + pair to resulting counts.
    // As such combination is unique and will repeat multiple times
    // it is used to reduce number of recursive calls.
    let mut mem: HashMap<(usize, String), [u64; 26]> = HashMap::new();

    // Count inserted elements for each pair recursively for a given amount of rounds.
    for i in 0..template.len() - 1 {
        let c = count_elements(&template[i..i + 2], rounds, &pair_map, &inserts, &mut mem);
        merge_counts(&mut counts, &c);
    }

    // Add characters of initial template to counts.
    for c in template.chars() {
        counts[(c as usize - 65)] += 1;
    }

    let max = counts.iter().max().unwrap();
    let min = counts.iter().filter(|x| **x > 0).min().unwrap();

    max - min
}

fn count_elements(
    pair: &str,
    rounds: usize,
    map: &HashMap<String, (String, String)>,
    inserts: &HashMap<String, char>,
    mem: &mut HashMap<(usize, String), [u64; 26]>,
) -> [u64; 26] {
    if rounds == 0 {
        return [0; 26];
    }

    match mem.get(&(rounds, pair.to_string())) {
        None => {}
        Some(counts) => return *counts,
    }

    let mut counts = [0; 26];

    let insert = inserts.get(pair).expect("expected to fin insert for pair");
    counts[(*insert as usize - 65)] += 1;

    let (p1, p2) = map.get(pair).expect("failed to find mapping");

    let c1 = count_elements(p1, rounds - 1, map, inserts, mem);
    let c2 = count_elements(p2, rounds - 1, map, inserts, mem);

    merge_counts(&mut counts, &c1);
    merge_counts(&mut counts, &c2);

    mem.insert((rounds, pair.to_string()), counts);

    counts
}

fn merge_counts(a: &mut [u64; 26], b: &[u64; 26]) {
    for (i, c) in b.iter().enumerate() {
        a[i] += *c;
    }
}

fn make_pair_map(
    pair: &str,
    inserts: &HashMap<String, char>,
    pair_map: &mut HashMap<String, (String, String)>,
) {
    match pair_map.get(pair) {
        None => {
            let mut chars = pair.chars();
            let insert_char = inserts.get(pair).expect("failed to find insert");

            let p1 = [chars.next().unwrap(), *insert_char]
                .iter()
                .collect::<String>();
            let p2 = [*insert_char, chars.next().unwrap()]
                .iter()
                .collect::<String>();

            pair_map.insert(pair.to_string(), (p1.clone(), p2.clone()));
            make_pair_map(&p1, inserts, pair_map);
            make_pair_map(&p2, inserts, pair_map);
        }
        Some(_) => {}
    }
}

fn read_data<P: AsRef<Path>>(input_file: P) -> (String, HashMap<String, char>) {
    let data = std::fs::read_to_string(input_file).unwrap();
    let (template, insertions) = data.split_once("\n\n").unwrap();

    let insertions: HashMap<String, char> = insertions
        .lines()
        .map(|insert| {
            let (k, v) = insert.split_once(" -> ").unwrap();
            (k.to_string(), v.chars().next().unwrap())
        })
        .collect();

    (template.to_string(), insertions)
}

#[cfg(test)]
mod test {
    use crate::day14::{day_14_1, day_14_2};
    use crate::util::temp_file_with_content;

    const TEST_DATA: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test() {
        let file = temp_file_with_content("day_14_01", TEST_DATA);
        assert_eq!(day_14_1(&file), 1588);
        assert_eq!(day_14_2(&file), 2188189693529);
    }
}
