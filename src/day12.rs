use crate::util::read_lines_raw;
use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
enum CaveSize {
    Small,
    Big,
}

#[derive(Clone, Debug)]
struct Cave {
    name: String,
    size: CaveSize,
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let start = s.chars().next().unwrap();
        let size = if start.is_ascii_uppercase() {
            CaveSize::Big
        } else {
            CaveSize::Small
        };

        Ok(Cave {
            name: s.to_string(),
            size,
        })
    }
}

pub fn day_12_1<P: AsRef<Path>>(input_file: P) -> usize {
    let caves_data = read_caves_data(input_file);
    let mut visited = HashMap::new();
    let start_cave = Cave {
        name: "start".to_owned(),
        size: CaveSize::Small,
    };

    // Originally went with HashSet for part 1. Added ugly bool to reuse code...
    count_paths(&start_cave, &caves_data, &mut visited, false)
}

pub fn day_12_2<P: AsRef<Path>>(input_file: P) -> usize {
    let caves_data = read_caves_data(input_file);
    let mut visited = HashMap::new();
    let start_cave = Cave {
        name: "start".to_owned(),
        size: CaveSize::Small,
    };

    count_paths(&start_cave, &caves_data, &mut visited, true)
}

// This could probably be optimized as it takes ~2 seconds to run part 2.
fn count_paths(
    cave: &Cave,
    caves_conns: &HashMap<String, Vec<Cave>>,
    cave_visited: &mut HashMap<String, u16>,
    allow_double: bool,
) -> usize {
    if cave_visited.get(&cave.name).is_some() {
        if !allow_double {
            // That is not to pretty but well... (:shrug:)
            return 0;
        }
        // If there is already a cave entered twice then stop.
        if cave_visited.values().any(|v| *v == 2) {
            return 0;
        }
    }
    if &cave.name == "end" {
        return 1;
    }
    if cave.size == CaveSize::Small {
        match cave_visited.get_mut(&cave.name) {
            None => {
                cave_visited.insert(cave.name.clone(), 1);
            }
            Some(val) => *val = 2,
        };
    }

    let paths = match caves_conns.get(&cave.name) {
        None => 0,
        Some(connections) => {
            let mut sum = 0;
            for c in connections {
                sum += count_paths(c, caves_conns, cave_visited, allow_double);
            }
            sum
        }
    };

    match cave_visited.get_mut(&cave.name) {
        None => {}
        Some(val) if *val == 2 => *val -= 1,
        Some(_) => {
            cave_visited.remove(&cave.name);
        }
    };

    paths
}

fn read_caves_data<P: AsRef<Path>>(input_file: P) -> HashMap<String, Vec<Cave>> {
    let cave_conns: Vec<String> = read_lines_raw(input_file);
    let mut conn_map = HashMap::new();

    for cc in cave_conns {
        let (from, to) = cc.split_once('-').unwrap();
        insert_connection(from, to, &mut conn_map);
        insert_connection(to, from, &mut conn_map);
    }

    conn_map
}

fn insert_connection(from: &str, to: &str, conn_map: &mut HashMap<String, Vec<Cave>>) {
    // Never treat start as a destination.
    if to == "start" {
        return;
    }

    let destination = Cave::from_str(to).unwrap();
    match conn_map.get_mut(from) {
        None => {
            let caves = vec![destination];
            conn_map.insert(from.to_owned(), caves);
        }
        Some(curr) => {
            curr.push(destination);
        }
    };
}

#[cfg(test)]
mod test {
    use crate::day12::{day_12_1, day_12_2};
    use crate::util::temp_file_with_content;

    const TEST_DATA_1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    const TEST_DATA_2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
    const TEST_DATA_3: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test() {
        let file1 = temp_file_with_content("day_12_01", TEST_DATA_1);
        let file2 = temp_file_with_content("day_12_02", TEST_DATA_2);
        let file3 = temp_file_with_content("day_12_03", TEST_DATA_3);

        assert_eq!(day_12_1(&file1), 10);
        assert_eq!(day_12_1(&file2), 19);
        assert_eq!(day_12_1(&file3), 226);
        assert_eq!(day_12_2(&file1), 36);
        assert_eq!(day_12_2(&file2), 103);
        assert_eq!(day_12_2(&file3), 3509);
    }
}
