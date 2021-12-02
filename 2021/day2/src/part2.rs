use crate::structs::{AimCoords};

pub fn part2(input: &Vec<(String, i32)>) {
    let mut coord = AimCoords::new();

    for line in input {
        match &line.0[..] {
            "forward" => coord.forwards(line.1),
            "up" => coord.up(line.1),
            "down" => coord.down(line.1),
            _ => ()
        }
    }

    println!("Part 2: {}", coord.result());
}