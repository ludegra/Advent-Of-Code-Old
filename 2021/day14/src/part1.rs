use std::collections::HashMap;

pub fn part1(base_polymer: &Vec<char>, insertion_pairs: &HashMap<(char, char), char>) {
    let mut base_polymer = base_polymer.clone();

    for _ in 0..10 {
        let mut polymer = Vec::new();
        polymer.push(base_polymer[0]);

        for component in &base_polymer[1..] {
            match insertion_pairs.get(&(*polymer.last().unwrap(), *component)) {
                Some(insertion) => {
                    polymer.push(*insertion);
                    polymer.push(*component);
                }
                None => {
                    polymer.push(*component);
                }
            }
        }
        base_polymer = polymer.clone();
    }

    let mut counter = HashMap::new();

    for component in &base_polymer {
        let entry = counter.entry(*component).or_insert(0);
        *entry += 1;
    }

    let mut counter: Vec<i32> = counter.values().map(|s| *s).collect();
    counter.sort();

    println!("Part 1: {}", counter.last().unwrap() - counter[0])
}