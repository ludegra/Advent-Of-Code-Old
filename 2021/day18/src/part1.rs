use crate::structs::PairComponent;

pub fn part1(input: &Vec<PairComponent>) {
    let mut iter = input.iter();
    let base = iter.next().unwrap().clone();
    let sum = iter.fold(base, |acc, s| acc + s.clone());

    println!("Part 1: {}", sum.magnitude());
}