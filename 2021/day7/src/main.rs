use part1::part1;
use part2::part2;

mod part1;
mod part2;
mod structs;

fn main() {
    let mut input = include_str!("../assets/input.txt").trim().split(',').map(|s| s.parse().unwrap()).collect::<Vec<i32>>();
    input.sort();

    part1(&input);
    part2(&input);
}
