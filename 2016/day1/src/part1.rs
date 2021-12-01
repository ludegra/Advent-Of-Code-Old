use crate::structs::{Coordinate, FacingDirection, TurningDirection};

pub fn part1(input: &Vec<(char, i32)>) {
    let mut coords = Coordinate::new();
    let mut facing = FacingDirection::Up;

    for (direction, length) in input {
        let turn = TurningDirection::match_char(*direction);
        facing = facing.turn(turn);
        coords.walk(facing, *length);
    }

    println!("Part1: {}", coords.distance());
}