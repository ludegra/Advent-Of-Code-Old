use structs::{Axis, Grid};

mod part1;
mod part2;
mod structs;

fn main() {
    let input = include_str!("../assets/input_ludegra.txt")
        .trim()
        .split("\n\n")
        .collect::<Vec<_>>();

    let grid = input[0]
        .split('\n')
        .fold(Grid::new(1311,894), |mut grid, s| {
            let split = s
                .split(',')
                .map(|i| i.parse().unwrap())
                .collect::<Vec<usize>>();
            grid.mark_point(split[0], split[1]);
            grid
        });

    let mut has_folded = false;

    let finished = input[1].split('\n').fold(grid, |mut grid, s| {
        let mut arg = s[11..].split('=');
        let axis = match arg.next().unwrap() {
            "x" => Axis::X,
            "y" => Axis::Y,
            _ => panic!("Invalid"),
        };
        let index = arg.next().unwrap().parse::<usize>().unwrap();
        grid.fold(axis, index);
        if !has_folded {
            println!("Part 1: {}", grid.count());
        }
        has_folded = true;
        grid
    });

    println!("Part 2:\n{}", finished);
}
