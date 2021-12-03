use part1::part1;
use part2::part2;

mod part1;
mod part2;
mod structs;

fn main() {
    let map_function = |s: &str| {
        s.chars().collect::<Vec<_>>()
    };

    let input = include_str!("../assets/input_ludegra.txt").split('\n').map(map_function).collect::<Vec<_>>();

    part1(&input);
    part2(&input);
}
