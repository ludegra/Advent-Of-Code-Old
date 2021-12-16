use structs::Package;

mod part2;
mod structs;

fn main() {
    let input = include_str!("../assets/test.txt").trim().chars().fold(String::new(), |mut acc, s| {
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
    println!("Part 2: {}", package.value());
}