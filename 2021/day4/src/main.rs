use std::time::Instant;

use part1::part1;
use part2::part2;
use regex::Regex;
use structs::Board;

mod part1;
mod part2;
mod structs;

fn main() {
    let start = Instant::now();
    let mut input = include_str!("../assets/input.txt").trim().split("\n\n");

    let numbers = input.next().unwrap().split(',').map(|s| { s.parse().unwrap() }).collect::<Vec<i32>>();

    let map_function = |s: &str| {
        let re = Regex::new(r" +").unwrap();
        let rows = s.split('\n').map(|s| {
            let s = s.trim();
            re.split(s).map(|s| s.parse().unwrap()).collect::<Vec<i32>>()
        }).collect::<Vec<Vec<i32>>>();

        Board::new(&rows[..])
    };

    let boards = input.map(map_function).collect::<Vec<Board>>();

    println!("Input processing: {}µs", start.elapsed().as_micros());
    let start = Instant::now();

    part1(&numbers, &boards);
    println!("Part 1 time: {}µs", start.elapsed().as_micros());
    let start = Instant::now();

    part2(&numbers, &boards);
    println!("Part 2 time: {}µs", start.elapsed().as_micros());
}