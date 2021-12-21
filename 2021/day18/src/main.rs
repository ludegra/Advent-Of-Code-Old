use std::str::Chars;

use part1::part1;
use part2::part2;
use structs::PairComponent;

mod part1;
mod part2;
mod structs;

fn main() {
    let input = include_str!("../assets/input_ludegra.txt")
        .trim()
        .split('\n')
        .map(|s| {
            map_string(&mut s.chars())
        })
        .collect::<Vec<_>>();

    part1(&input);
    part2(&input);
}

fn map_string(chars: &mut Chars) -> PairComponent {
    let next = chars.next().unwrap();

    let out = match next {
        '[' => {
            let left = map_string(chars);
            chars.next().unwrap(); // Comma
            let right = map_string(chars);
            chars.next().unwrap(); // Closing bracket
            PairComponent::Pair(Box::new(left), Box::new(right))
        }
        '0'..='9' => PairComponent::Value(next.to_digit(10).unwrap()),
        _ => panic!("Invalid character")
    };  
    out
}
