use std::time::Instant;

use part1::part1;
use part2::part2;

mod part1;
mod part2;
mod structs;

fn main() {
    let start = Instant::now();
    let input = include_str!("../assets/input.txt")
        .trim()
        .split('\n')
        .map(|s| {
            let mut split = s.split(" | ")
                .map(|s| {
                    s.trim().split(' ').map(|s| s.to_string()).collect::<Vec<String>>()
                })
                .collect::<Vec<_>>();
            split[0].sort_by(|a, b| a.len().cmp(&b.len()));
            (split[0].clone(), split[1].clone())
        })
        .collect::<Vec<_>>();

    println!("Input time: {}µs", start.elapsed().as_micros());
    println!("--------------------");
    let start = Instant::now();

    part1(&input);

    println!("Part 1 time: {}µs", start.elapsed().as_micros());
    println!("--------------------");
    let start = Instant::now();

    part2(&input);

    println!("Part 2 time: {}µs", start.elapsed().as_micros());
    println!("--------------------");
}
