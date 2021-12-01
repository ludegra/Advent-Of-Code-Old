use std::collections::HashMap;

use crate::structs::Room;

pub fn part1(input: &Vec<Room>) {
    
    let mut counter = 0;

    'outer: for room in input {
        let mut map = HashMap::new();

        for letter in room.name.chars() {
            let entry = map.entry(letter).or_insert(0);

            *entry += 1;
        }

        let mut kv_pairs: Vec<(char, i32)> = map.into_iter().collect();
        kv_pairs.sort_by(|a,b| (b.1, a.0).cmp(&(a.1, b.0)));

        let mut checksum = room.checksum.chars();

        for i in 0..room.checksum.len() {
            let next = checksum.next().unwrap();
            if kv_pairs[i].0 != next { continue 'outer }
        }
        counter += room.id;
    }
    
    println!("Part 1: {}", counter);
}