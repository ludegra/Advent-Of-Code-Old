use structs::Package;

use crate::structs2::Package2;

mod part2;
mod structs;
mod structs2;

fn main() {
    let input = include_str!("../assets/input_ludegra.txt").trim().chars().fold(String::new(), |mut acc, s| {
        let binary = format!("{:b}", s.to_digit(16).unwrap());
        for _ in 0..(4 - binary.len()) {
            acc.push('0');
        }
        acc.push_str(&binary);
        acc
    });
    println!("{}", input);
    let (package, _) = Package::new(&input);

    println!("{:?}", package);

    println!("Part 1: {}", package.sum_versions());

    let (package, _) = Package2::new(&input);
    println!("Part 2: {}", package.value());
}