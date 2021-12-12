use part1::part1;
use part2::part2;

mod part1;
mod part2;
mod structs;

fn main() {
    let input = include_str!("../assets/input_ludegra.txt").trim().split("\n").collect::<Vec<_>>();
    let mut legal_lines = Vec::new();

    part1(&input, &mut legal_lines);
    part2(legal_lines);
}
