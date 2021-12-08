use std::collections::HashMap;

pub fn part2(input: &Vec<i32>) {
    let mut hash = HashMap::new();

    for element in input {
        let entry = hash.entry(*element).or_insert(0);
        *entry += 1;
    }

    let mut smallest = i32::MAX;

    for destination in 0..(*input.last().unwrap()) {
        let mut sum = 0;
        for (position, number) in &hash {
            let n = (position - destination).abs();
            sum += number * (n * (n + 1)) / 2
        }
        if sum < smallest {
            smallest = sum;
        }
    }

    println!("Part 2: {}", smallest);
}