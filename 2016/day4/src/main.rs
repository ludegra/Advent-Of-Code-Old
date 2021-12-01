use part1::part1;
use part2::part2;
use structs::Room;
use utils::input::read_input_lines;

mod part1;
mod part2;
mod structs;

fn main() {
    let map_function = |s: &String| {
        let len = s.len();

        let checksum = s[len-6..len-1].to_string();
        let id = s[len-10..len-7].parse().unwrap();
        let name = s[..len-11].replace("-", "");

        Room {
            name, id, checksum,
        }
    };

    let input: Vec<Room> = read_input_lines("assets/input.txt").unwrap().iter().map(map_function).collect();

    part1(&input);
    part2(&input);
}
