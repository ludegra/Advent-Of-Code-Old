use part1::part1;
use part2::part2;

mod part1;
mod part2;
mod structs;

fn main() {
    let input = include_str!("../assets/input_ludegra.txt")
        .trim()
        .split('\n')
        .map(|s| {
            s.chars()
                .map(|s| s.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    part1(&input);
    part2(&input);
}
