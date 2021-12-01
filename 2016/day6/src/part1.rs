use std::collections::HashMap;

pub fn part1(input: &Vec<String>) {
    let mut maps = vec![HashMap::new(); input[0].len()];

    for line in input {
        let mut chars = line.chars();

        for i in 0..line.len() {
            let character = chars.next().unwrap();
            let entry = maps[i].entry(character).or_insert(0);
            *entry += 1;
        }
    }

    let mut output1 = String::new();
    let mut output2 = String::new();

    for position in maps {
        let mut vector: Vec<(char, i32)> = position.iter().map(|s| (s.0.clone(), s.1.clone())).collect();
        vector.sort_by(|a, b| a.1.cmp(&b.1));
        output2.push(vector[0].0);

        vector = vector.iter().rev().map(|s|s.clone()).collect();
        output1.push(vector[0].0);
    }

    println!("Part 1: {}", output1);
    println!("Part 2: {}", output2);
}