use crate::structs::WeirdPad;

pub fn part2(input: &Vec<String>) {
    let mut code = String::new();
    for instructions in input {
        let chars = instructions.chars();
        let mut pad = WeirdPad::new();

        for command in chars {
            pad.move_key(command);
        }

        code.push(pad.get_key());
    }

    println!("Part2: {}", code);
}