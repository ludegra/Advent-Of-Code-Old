mod structs;
mod part1;
mod part2;

use part1::part1;
use part2::part2;
use utils::input::read_input_lines;

fn main() {
    let map_function = |s: &String| -> (String, i32) {
        let split = s.split(' ').collect::<Vec<_>>();

        (split[0].clone().to_string(), split[1].parse().unwrap())
    };

    let input = read_input_lines("assets/input.txt").unwrap().iter().map(map_function).collect::<Vec<_>>();

    part1(&input);
    part2(&input);
}
