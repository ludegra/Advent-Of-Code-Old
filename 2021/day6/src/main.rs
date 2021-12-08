use std::time::Instant;

use part1::part1;
use part2::part2;

mod part1;
mod part2;
mod structs;

fn main() {
    let start = Instant::now();
    let input = include_str!("../assets/input_ludegra.txt").trim().split(',').map(|s| s.parse().unwrap()).collect::<Vec<u32>>();

    println!("Input time: {}µs", start.elapsed().as_micros());
    let start = Instant::now();

    part1(&input);

    println!("Part 1 time: {}µs", start.elapsed().as_micros());
    let start = Instant::now();

    part2(&input);

    println!("Part 2 time: {}µs", start.elapsed().as_micros());
}
