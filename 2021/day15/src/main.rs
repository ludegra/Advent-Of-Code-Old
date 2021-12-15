use part1::part1;
use part2::part2;
use structs::Point;

mod part1;
mod part2;
mod structs;

fn main() {
    let (mut x, mut y) = (0, 0);
    let input = include_str!("../assets/test.txt")
        .trim()
        .split('\n')
        .map(|s| {
            let row = s
                .chars()
                .map(|i| {
                    let value = i.to_digit(10).unwrap();
                    let point = Point::new(x, y, value);
                    x += 1;
                    point
                })
                .collect::<Vec<_>>();
            x = 0;
            y += 1;
            row
        })
        .collect::<Vec<_>>();

    part1(&input);
    part2(&input);
}
