use part1::part1;
use part2::part2;
use structs::{Point, Scanner};

mod part1;
mod part2;
mod structs;

fn main() {
    let mut index = 0;
    let input = include_str!("../assets/input_ludegra.txt")
        .trim()
        .split("\n\n")
        .map(|s| {
            let mut iterator = s.split('\n');
            iterator.next().unwrap(); // Removing headings

            let beacons = iterator
                .map(|s| {
                    let mut split = s.split(',');

                    Point {
                        x: split.next().unwrap().parse().unwrap(),
                        y: split.next().unwrap().parse().unwrap(),
                        z: split.next().unwrap().parse().unwrap(),
                    }
                })
                .collect::<Vec<_>>();

            let scanner = Scanner::new(index, &beacons);
            index += 1;
            scanner
        })
        .collect::<Vec<_>>();

    let positions = part1(&input);
    part2(positions);
}
