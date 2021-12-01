mod part1;
mod part2;
mod structs;

use part1::part1;
use utils::input::read_input_lines;

fn main() {
    let input = read_input_lines("assets/input.txt").unwrap();

    part1(&input);
}
