use part1::part1;
use part2::part2;

mod part1;
mod part2;

fn main() {
    let input = include_str!("../assets/input.txt")
        .trim()
        .split('\n')
        .map(|s| {
            let mut split = s.split(' ');

            let on = match split.next().unwrap() {
                "on" => true,
                "off" => false,
                _ => panic!("Invalid start of row"),
            };

            let values = split
                .next()
                .unwrap()
                .split(',')
                .map(|s| {
                    let vec = s[2..]
                        .split("..")
                        .map(|i| i.parse::<isize>().unwrap())
                        .collect::<Vec<_>>();
                    (vec[0], vec[1])
                })
                .collect::<Vec<_>>();

            (on, values[0].clone(), values[1].clone(), values[2].clone())
        })
        .collect::<Vec<_>>();

    part1(&input);
    part2(&input)
}
