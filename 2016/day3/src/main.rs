use part1::part1;
use part2::part2;
use utils::input::read_input_lines;
use regex::Regex;

mod part1;
mod part2;
mod structs;

fn main() {
    let split_re = Regex::new(r" +").unwrap();

    let map_function = |s: &String| {
        let trimmed = s.trim();
        let split = split_re.split(trimmed);
        split.map(|i| i.parse().unwrap()).collect()
    };

    let mut input: Vec<Vec<i32>> = read_input_lines("assets/input.txt").unwrap().iter().map(map_function).collect();

    part1(&mut input.clone());
    part2(&mut input);
}
