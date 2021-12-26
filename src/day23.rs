use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::fs;
use std::path::Path;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl PartialOrd for Amphipod {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Amphipod {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost().cmp(&other.cost())
    }
}

impl From<char> for Amphipod {
    fn from(c: char) -> Self {
        match c {
            'A' => Amphipod::Amber,
            'B' => Amphipod::Bronze,
            'C' => Amphipod::Copper,
            'D' => Amphipod::Desert,
            _ => unreachable!("unexpected char {}", c),
        }
    }
}

impl Amphipod {
    fn correct_room(&self) -> usize {
        match self {
            Amphipod::Amber => 0,
            Amphipod::Bronze => 1,
            Amphipod::Copper => 2,
            Amphipod::Desert => 3,
        }
    }

    fn cost(&self) -> usize {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }
}

#[derive(PartialOrd, PartialEq, Eq, Ord, Clone, Copy, Debug, Hash)]
struct State<const N: usize> {
    rooms: [[Option<Amphipod>; N]; 4],
    hallway: [Option<Amphipod>; 7],
}

impl<const N: usize> State<N> {
    fn new() -> Self {
        Self {
            rooms: [[None; N]; 4],
            hallway: [None; 7],
        }
    }
}

const HALLWAY_COLS: [i32; 7] = [0, 1, 3, 5, 7, 9, 10];
const ROOM_COLS: [i32; 4] = [2, 4, 6, 8];
const ROOM_HALLWAYS: [(usize, usize); 4] = [(1, 2), (2, 3), (3, 4), (4, 5)];
const COSTS: [i32; 4] = [1, 10, 100, 1000];

fn heuristics<const N: usize>(state: State<N>) -> i32 {
    // Cost for Amphipods in hallway
    let hallway_cost: i32 = state
        .hallway
        .iter()
        .enumerate()
        .filter_map(|(n, spot)| {
            spot.map(|pod| {
                ((ROOM_COLS[pod as usize] - HALLWAY_COLS[n]).abs() + 1) * COSTS[pod as usize]
            })
        })
        .sum();

    // Cost for Amphipods in rooms
    let rooms_cost: i32 = state
        .rooms
        .iter()
        .enumerate()
        .map(|(n, room)| {
            room.iter()
                .enumerate()
                .filter_map(|(m, spot)| {
                    spot.and_then(|pod| {
                        if pod.correct_room() != n {
                            Some(
                                ((ROOM_COLS[pod as usize] - ROOM_COLS[n]).abs() + m as i32 + 2)
                                    * COSTS[pod as usize],
                            )
                        } else {
                            None
                        }
                    })
                })
                .sum::<i32>()
        })
        .sum();

    hallway_cost + rooms_cost
}

fn find_cost<const N: usize>(initial_state: State<N>) -> u64 {
    let mut open_set = BinaryHeap::new();
    open_set.push((Reverse(0), 0, initial_state));

    let mut checked = HashMap::new();

    while let Some((_, cost, state)) = open_set.pop() {
        if checked
            .get(&state)
            .map_or(false, |&old_cost| cost >= old_cost)
        {
            continue;
        }
        checked.insert(state, cost);

        // If all are in correct room, we stop
        if state
            .rooms
            .iter()
            .enumerate()
            .all(|(n, room)| room.iter().all(|x| x.map(|x| x.correct_room()) == Some(n)))
        {
            return cost;
        }

        for (room_id, room) in state.rooms.iter().enumerate() {
            // If all are correct in this room go to the next
            if !room
                .iter()
                .any(|pod| pod.map_or(false, |pod| pod.correct_room() != room_id))
            {
                continue;
            }
            let position = room
                .iter()
                .enumerate()
                .find_map(|(y, pod)| if pod.is_some() { Some(y) } else { None })
                .unwrap();

            for i in (0..)
                .map(|i| ROOM_HALLWAYS[room_id].1 + i)
                .take_while(|&i| state.hallway.get(i) == Some(&None))
                .chain(
                    (0..)
                        .take_while(|&i| {
                            i <= ROOM_HALLWAYS[room_id].0
                                && state.hallway[ROOM_HALLWAYS[room_id].0 - i].is_none()
                        })
                        .map(|i| ROOM_HALLWAYS[room_id].0 - i),
                )
            {
                let mut new_state = state;
                std::mem::swap(
                    &mut new_state.rooms[room_id][position],
                    &mut new_state.hallway[i],
                );
                let new_cost = cost as i32
                    + (position as i32 + 1 + (ROOM_COLS[room_id] - HALLWAY_COLS[i]).abs())
                        * COSTS[state.rooms[room_id][position].unwrap() as usize];
                let f_score = new_cost + heuristics(new_state);

                open_set.push((Reverse(f_score), new_cost as u64, new_state));
            }
        }

        for (x, spot) in state.hallway.iter().enumerate() {
            if let Some(pod) = spot {
                let pod = *pod;
                let room = state.rooms[pod as usize];
                if (room
                    .iter()
                    .all(|spot| spot.map_or(true, |other_pod| other_pod == pod)))
                    && (if x >= ROOM_HALLWAYS[pod as usize].1 {
                        ROOM_HALLWAYS[pod as usize].1..x
                    } else {
                        x + 1..ROOM_HALLWAYS[pod as usize].0 + 1
                    })
                    .all(|x| state.hallway[x].is_none())
                {
                    let y = (0..room.len())
                        .rev()
                        .find(|&spot| room[spot].is_none())
                        .unwrap();

                    let mut new_state = state;
                    std::mem::swap(
                        &mut new_state.rooms[pod as usize][y],
                        &mut new_state.hallway[x],
                    );

                    let new_cost = cost as i32
                        + (y as i32
                            + 1
                            + (ROOM_COLS[pod as usize] - HALLWAY_COLS[x as usize]).abs())
                            * COSTS[pod as usize];

                    let f_score = new_cost + heuristics(new_state);

                    open_set.push((Reverse(f_score), new_cost as u64, new_state));
                }
            }
        }
    }
    panic!("failed to find cost")
}

fn parse_data<const N: usize>(lines: Vec<&str>) -> State<N> {
    let mut initial_state: State<N> = State::new();
    for (n, line) in lines.iter().skip(2).enumerate() {
        for (m, c) in line.chars().filter(|c| c.is_ascii_uppercase()).enumerate() {
            initial_state.rooms[m][n] = Some(c.into());
        }
    }
    initial_state
}

pub fn day_23_1<P: AsRef<Path>>(input_file: P) -> u64 {
    let data = fs::read_to_string(input_file).unwrap();
    let lines: Vec<&str> = data.lines().collect();
    let initial_state = parse_data::<2>(lines);
    find_cost::<2>(initial_state)
}

pub fn day_23_2<P: AsRef<Path>>(input_file: P) -> u64 {
    let data = fs::read_to_string(input_file).unwrap();
    let mut lines: Vec<&str> = data.lines().collect();
    lines.insert(3, "#D#C#B#A#");
    lines.insert(4, "#D#B#A#C#");
    let initial_state = parse_data::<4>(lines);
    find_cost::<4>(initial_state)
}

#[cfg(test)]
mod test {
    use crate::day23::{day_23_1, day_23_2};
    use crate::util::temp_file_with_content;

    const TEST_DATA: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    fn test() {
        let file = temp_file_with_content("day_23", TEST_DATA);

        assert_eq!(day_23_1(&file), 12521);
        assert_eq!(day_23_2(&file), 44169);
    }
}
