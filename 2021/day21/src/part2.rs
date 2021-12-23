use std::{collections::HashMap};

// (Roll, Probability)
const CASES: [(u32, u64); 7] = [
    (3, 1),
    (4, 3),
    (5, 6),
    (6, 7),
    (7, 6),
    (8, 3),
    (9, 1)
];

pub fn part2(player1_pos: u32, player2_pos: u32) {

    // (Score P1, Score P2) -> (Position P1, Position P2, Number)
    let mut score_counter = HashMap::new();
    score_counter.insert((0u32, 0u32, player1_pos, player2_pos), 1u64);

    let mut win_counter = (0u64, 0u64);

    play_game(score_counter, &mut win_counter);

    println!("Part 2: {}", win_counter.0);

}

fn play_game(mut score_counter: HashMap<(u32, u32, u32, u32), u64>, win_counter: &mut (u64, u64)) {

    while !score_counter.is_empty() {

        let mut round_score = HashMap::new();

        for ((p1_score, p2_score, p1_pos, p2_pos), number) in &score_counter {
        
            // (Score, Number)
            let mut p1_results = Vec::new();

            for (dice_roll, probability) in CASES {

                let position = roll_dice(*p1_pos, dice_roll);
                let score = position + p1_score;

                if score >= 21 {
                    win_counter.0 += number * probability
                }
                else {
                    p1_results.push((score, position, number * probability));
                }
            }
            
            for (p1_score, p1_pos, number) in p1_results {

                for (dice_roll, probability) in CASES {

                    let position = roll_dice(*p2_pos, dice_roll);
                    let score = position + p2_score;

                    if score >= 21 {
                        win_counter.1 += number * probability;
                    }
                    else {
                        let entry = round_score.entry((p1_score, score, p1_pos, position)).or_insert(0);

                        *entry += number * probability;
                    }

                }
            }

            // let mut vec = round_score.iter().map(|(k, v)| ((k.0, k.1), (v.0, v.1, v.2))).collect::<Vec<_>>();
            // vec.sort();
            // for (k, v) in &vec {
            //     println!("{:?} -> {:?}", k, v);
            // }
            // println!("{}", vec.iter().fold(0, |acc, s| acc + s.1.2));
            // panic!()
        }
        score_counter = round_score.clone();
    }

}
 
fn roll_dice(player_pos: u32, dice_roll: u32) -> u32 {
    let mut result = (player_pos + dice_roll) % 10;
    if result == 0 {
        result = 10;
    }
    result
}