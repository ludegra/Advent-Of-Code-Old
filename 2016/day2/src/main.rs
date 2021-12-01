use part1::part1;
use part2::part2;
use utils::input::read_input_lines;

mod part1;
mod part2;
mod structs;

fn main() {
    let input = read_input_lines("assets/input.txt").unwrap();

    part1(&input);
    part2(&input);
}
