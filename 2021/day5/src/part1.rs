use crate::structs::Grid;

pub fn part1(input: &Vec<((usize, usize), (usize, usize))>) {
    let mut grid = Grid::new();

    for (start, end) in input.clone() {
        grid.mark_straight_line(start, end);
    }

    println!("Part 1: {}", grid.number_2_or_more());
}