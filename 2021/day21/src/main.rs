use part1::part1;
use part2::part2;

mod part1;
mod part2;

fn main() {
    let input = include_str!("../assets/input_ludegra.txt").trim().split('\n').map(|s| s["Player 1 starting position: ".len()..].parse::<u32>().unwrap()).collect::<Vec<_>>();
    let mut input = input.iter();
    let player1_pos = *input.next().unwrap();
    let player2_pos = *input.next().unwrap();

    part1(player1_pos, player2_pos);
    part2(player1_pos, player2_pos);
}