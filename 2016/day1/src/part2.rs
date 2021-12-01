use crate::structs::{Coordinate, FacingDirection, TurningDirection};

pub fn part2(input: &Vec<(char, i32)>) {
    let mut coords = Coordinate::new();
    let mut facing = FacingDirection::Up;
    let mut places = Vec::new();

    'outer: for (direction, length) in input {
        let turn = TurningDirection::match_char(*direction);
        facing = facing.turn(turn);
        
        for _ in 0..*length {
            coords.walk(facing, 1);
            let temp_coords = coords.get_coords();
            
            if places.contains(&temp_coords) {
                break 'outer
            }
            else {
                places.push(temp_coords)
            }
        }
    }

    println!("Part2: {}", coords.distance());
}