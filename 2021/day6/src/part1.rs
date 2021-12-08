use std::collections::HashMap;

use crate::structs::Fish;

pub fn part1(input: &Vec<u32>) {
    let mut checker: HashMap<i32, Option<u64>> = HashMap::new();
    let mut sum = 0;

    for i in input {
        let fish = Fish::new(*i as i32, 80);
        sum += fish.count_children(&mut checker);
    }

    println!("Part 1: {}", sum);
}