use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// MOVES_WEIGHT represents how many times the outcome of 3 throws can occur.
/// For example 3 can occur only in once case when subsequent throws are 1, 1, 1.
static MOVES_WEIGHT: [(usize, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn new(pos: usize) -> Player {
        Player {
            position: pos,
            score: 0,
        }
    }

    fn move_forward(&mut self, len: usize) {
        let mut next_pos = len + self.position;
        while next_pos > 10 {
            next_pos = next_pos % 11 + next_pos / 11;
        }

        self.position = next_pos;
        self.score += self.position
    }
}

pub fn day_21_1<P: AsRef<Path>>(input_file: P) -> usize {
    let data = fs::read_to_string(input_file).expect("failed to read input file");
    let mut players = read_players(&data);

    let mut player_turn = 0;
    let mut dice_rolls = 0;
    let mut dice = 0;

    while players[0].score < 1000 && players[1].score < 1000 {
        let mut move_len = 0;
        for _i in 0..3 {
            dice = (dice) % 100 + 1;
            move_len += dice;
        }
        dice_rolls += 3;

        players[player_turn].move_forward(move_len);
        player_turn = (player_turn + 1) % 2;
    }

    players[0].score.min(players[1].score) * dice_rolls
}

pub fn day_21_2<P: AsRef<Path>>(input_file: P) -> usize {
    let data = fs::read_to_string(input_file).expect("failed to read input file");
    let players = read_players(&data);

    let mut mem = HashMap::new();

    let (score_1, score_2) = play_dirac_dice_game(players, 0, &mut mem);

    score_1.max(score_2)
}

fn play_dirac_dice_game(
    players: [Player; 2],
    turn: usize,
    mem: &mut HashMap<([Player; 2], usize), (usize, usize)>,
) -> (usize, usize) {
    if let Some(score) = mem.get(&(players, turn)) {
        return *score;
    }

    if players[0].score >= 21 {
        return (1, 0);
    }
    if players[1].score >= 21 {
        return (0, 1);
    }

    let scores: (usize, usize) = MOVES_WEIGHT
        .iter()
        .map(|&player_move| {
            let mut players = players;
            players[turn].move_forward(player_move.0);

            let (p1, p2) = play_dirac_dice_game(players, (turn + 1) % 2, mem);
            (p1 * player_move.1, p2 * player_move.1)
        })
        .fold((0, 0), |(sum_p1, sum_p2), (p1, p2)| {
            (sum_p1 + p1, sum_p2 + p2)
        });

    mem.insert((players, turn), scores);

    scores
}

fn read_players(data: &str) -> [Player; 2] {
    let (p1, p2) = data
        .split_once('\n')
        .map(|(l1, l2)| (player_position(l1), player_position(l2)))
        .unwrap();

    [Player::new(p1), Player::new(p2)]
}

fn player_position(data: &str) -> usize {
    data.chars().last().unwrap().to_digit(10).unwrap() as usize
}

#[cfg(test)]
mod test {
    use crate::day21::{day_21_1, day_21_2};
    use crate::util::temp_file_with_content;

    const TEST_DATA: &str = "Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn test() {
        let file = temp_file_with_content("day_21", TEST_DATA);

        assert_eq!(day_21_1(&file), 739785);
        assert_eq!(day_21_2(&file), 444356092776315);
    }
}
