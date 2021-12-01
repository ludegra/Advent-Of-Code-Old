use part1::part1;
use part2::part2;

mod part1;
mod part2;
mod structs;

fn main() {
    let map_function = |s: &str| {
        let dir = s.chars().next().unwrap();
        let len: i32 = s[1..].parse().unwrap();
        (dir, len)
    };
    let input: Vec<(char, i32)> = utils::input::read_input("assets/input.txt").unwrap().split(", ").map(map_function).collect();
    part1(&input);
    part2(&input);   
}
