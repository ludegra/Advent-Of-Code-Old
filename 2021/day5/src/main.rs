use part1::part1;
use part2::part2;

mod part1;
mod part2;
mod structs;

fn main() {
    let input = include_str!("../assets/input_ludegra.txt").trim().split('\n').map(|s| {
        let coords = s.split(" -> ").map(|s| {
            let coords = s.split(',').map(|s| s.parse().unwrap()).collect::<Vec<usize>>();
            (coords[0], coords[1])
        }).collect::<Vec<_>>();
        (coords[0], coords[1])
    }).collect::<Vec<_>>();

    part1(&input);
    part2(&input);
}