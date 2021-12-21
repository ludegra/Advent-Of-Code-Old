pub fn part1(mut player1_pos: u32, mut player2_pos: u32) {
    let mut player1_score = 0;
    let mut player2_score = 0;

    let mut dice_tosses = 0;

    let mut result = 0;

    for i in (2..).step_by(6) {
        let i = i % 100;
        if let Some(score) = increase_score(
            &mut player1_pos,
            &mut player1_score,
            i,
            &mut dice_tosses,
            &player2_score,
        ) {
            result = score;
            break;
        }
        if let Some(score) = increase_score(
            &mut player2_pos,
            &mut player2_score,
            i + 3,
            &mut dice_tosses,
            &player1_score,
        ) {
            result = score;
            break;
        }
    }

    println!("Part 1: {}", result);
}

fn increase_score(
    player_pos: &mut u32,
    player_score: &mut u32,
    i: u32,
    dice_tosses: &mut u32,
    other_score: &u32,
) -> Option<u32> {
    if i == 1 {
        *player_pos += 100 + 1 + 2
    } else if i == 100 {
        *player_pos += 99 + 100 + 1
    } else {
        *player_pos += 3 * i;
    }

    while *player_pos > 10 {
        *player_pos -= 10;
    }
    *player_score += *player_pos;

    *dice_tosses += 3;

    if *player_score >= 1000 {
        return Some(*other_score * *dice_tosses);
    }
    None
}
