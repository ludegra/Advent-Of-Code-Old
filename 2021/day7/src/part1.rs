use std::collections::HashMap;

pub fn part1(input: &Vec<i32>) {
    let mut hash = HashMap::new();

    for element in input {
        let entry = hash.entry(*element).or_insert(0);
        *entry += 1;
    }

    let mut smallest = i32::MAX;

    for destination in 0..(*input.last().unwrap()) {
        let mut sum = 0;
        for (position, number) in &hash {
            sum += number * (position - destination).abs()
        }
        if sum < smallest {
            smallest = sum;
        }
    }

    println!("Part 1: {}", smallest);
}