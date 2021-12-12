use part1::part1;
use part2::part2;
use structs::Point;

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

    let mut low_points = Vec::<Point>::new();

    part1(&input, &mut low_points);
    part2(input, low_points);
}
