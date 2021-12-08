use crate::structs::Grid;

pub fn part2(input: &Vec<((usize, usize), (usize, usize))>) {
    let mut grid = Grid::new();

    for (start, end) in input.clone() {
        grid.mark_line(start, end);
        // println!("{:?} -> {:?}", start, end);
        // println!("{}", grid);
    }

    println!("Part 2: {}", grid.number_2_or_more());
}