use std::time::Instant;

use part1::part1;
use part2::part2;

mod part1;
mod part2;
mod structs;

fn main() {
    let start = Instant::now();
    let map_function = |s: &str| {
        s.chars().collect::<Vec<_>>()
    };

    let input = include_str!("../assets/input.txt").split("\r\n").map(map_function).collect::<Vec<_>>();

    println!("Startup time: {}µs\n", start.elapsed().as_micros());
    let start = Instant::now();

    part1(&input);
    println!("Part 1 time: {}µs\n", start.elapsed().as_micros());
    let start = Instant::now();

    part2(&input);
    println!("Part 2 time: {}µs", start.elapsed().as_micros());
}
