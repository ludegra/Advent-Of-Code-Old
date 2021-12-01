use part1::part1;
use regex::Regex;
use utils::input::read_input_lines;

mod part1;
mod part2;
mod structs;

fn main() {
    let parser = |s: &String| {
        let re = Regex::new(r"(\[|\])").unwrap();
        let split = re.split(s).collect::<Vec<_>>();

        ((split[0].to_string(), split[2].to_string()), split[1].to_string())
    };

    let input = read_input_lines("assets/test.txt").unwrap().iter().map(parser).collect::<Vec<_>>();

    part1(&input);
}
