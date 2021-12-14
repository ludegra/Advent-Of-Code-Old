use std::collections::HashMap;

use part1::part1;
use part2::part2;

mod part1;
mod part2;

fn main() {
    let polymer: Vec<char>;
    let mut insertion_pairs: HashMap<(char, char), char> = HashMap::new();

    let mut input = include_str!("../assets/input.txt").trim().split("\n\n");
    polymer = input.next().unwrap().chars().collect::<Vec<_>>();
    input.next().unwrap().split('\n').for_each(|x| {
        let chars = x.chars().collect::<Vec<_>>();
        insertion_pairs.insert((chars[0], chars[1]), chars[6]);
    });

    part1(&polymer, &insertion_pairs);
    part2(&polymer, &insertion_pairs)
}
