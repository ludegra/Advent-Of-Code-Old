use crate::structs::PairComponent;

pub fn part2(input: &Vec<PairComponent>) {
    let mut biggest = 0;

    for term1 in input {
        for term2 in input {
            let magnitude = (term1.clone() + term2.clone()).magnitude();
            if magnitude > biggest {
                biggest = magnitude;
            }
        }
    }
    println!("Part 2: {}", biggest);
}