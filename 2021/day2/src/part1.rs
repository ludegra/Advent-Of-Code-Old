use crate::structs::Coordinate;

pub fn part1(input: &Vec<(String, i32)>) {
    let mut coord = Coordinate::new();

    for line in input {
        match &line.0[..] {
            "forward" => coord.forwards(line.1),
            "up" => coord.up(line.1),
            "down" => coord.down(line.1),
            _ => ()
        }
    }

    println!("Part 1: {}", coord.result());
}