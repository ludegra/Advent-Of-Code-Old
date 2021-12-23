use crate::structs::{Scanner, Point};

pub fn part1(input: &Vec<Scanner>) -> Vec<Point> {
    let mut scanner_of_comparison = input[0].clone();
    let mut positions = Vec::new();

    let mut has_changed = true;
    while has_changed {
        let old = scanner_of_comparison.clone();
        'outer: for scanner in input[1..].to_vec() {
            for mut rotation in scanner.all_rotations() {
                if let Some(diff) = scanner_of_comparison.compare_points(&mut rotation) {
                    if !positions.contains(&diff) {
                        positions.push(diff);
                    }
                    continue 'outer;
                }
            }
        }
        has_changed = !(old == scanner_of_comparison);
    }
    println!("\nPart 1: {}", scanner_of_comparison.number_beacons());
    positions
}
