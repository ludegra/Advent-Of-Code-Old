mod structs;
mod part1;
mod part2;

use std::time::Instant;

use part1::part1;
use part2::part2;
use utils::input::read_input_lines;

fn main() {
    let now = Instant::now();
    let map_function = |s: &String| -> (String, i32) {
        let split = s.split(' ').collect::<Vec<_>>();

        (split[0].clone().to_string(), split[1].parse().unwrap())
    };

    let input = read_input_lines("assets/input.txt").unwrap().iter().map(map_function).collect::<Vec<_>>();

    let input_time = now.elapsed();
    println!("Input time: {}µs\n", input_time.as_micros());
    let now = Instant::now();

    part1(&input);
    let part1_time = now.elapsed();
    println!("Part 1 time: {}µs\n", part1_time.as_micros());
    let now = Instant::now();

    part2(&input);
    let part2_time = now.elapsed();
    println!("Part 2 time: {}µs", part2_time.as_micros());
}
