use std::collections::HashMap;

use part2::part2;
use structs::Cave;

use crate::part1::part1;

mod part1;
mod part2;
mod structs;

fn main() {
    let input = include_str!("../assets/input_ludegra.txt")
        .trim()
        .split('\n')
        .map(|s| {
            let split = s.split('-').collect::<Vec<_>>();
            (split[0].to_string(), split[1].to_string())
        })
        .collect::<Vec<_>>();

    let mut caves = HashMap::with_capacity(input.len());

    for (source_name, target_name) in &input {
        let target_cave = caves
            .entry(target_name.clone())
            .or_insert(Cave::new(target_name.clone()));

        target_cave.add_connection(source_name.clone());

        let source_cave = caves
            .entry(source_name.clone())
            .or_insert(Cave::new(source_name.clone()));

        source_cave.add_connection(target_name.clone());
    }

    part1(&caves);
    part2(&caves);
}
