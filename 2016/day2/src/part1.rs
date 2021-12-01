use crate::structs::KeyPad;

pub fn part1(input: &Vec<String>) {
    let mut code = String::new();
    for instructions in input {
        let chars = instructions.chars();
        let mut pad = KeyPad::new();

        for command in chars {
            pad.move_key(command);
        }

        code.push(pad.get_key());
    }

    println!("Part1: {}", code);
}