use crate::{
    part1::part1,
    structs::{Point, Print},
};

pub fn part2(input: &[Vec<Point>]) {
    let mut grid = Vec::with_capacity(input.len());

    for row in input {
        let mut true_row = Vec::with_capacity(row.len() * 5);
        let row_len = row.len();
        for modifyer in 0..5 {
            true_row.append(
                &mut row
                    .iter()
                    .map(|s| {
                        let mut new_value = s.original_value + modifyer;
                        if new_value > 9 {
                            new_value -= 9;
                        }

                        Point::new(s.x + row_len, s.y, new_value)
                    })
                    .collect::<Vec<Point>>(),
            );
        }
        grid.push(true_row);
    }
    // grid.print(1);

    let mut true_grid = Vec::with_capacity(input.len() * 5);

    let grid_len = grid.len();
    for modifyer in 0..5 {
        for row in grid.clone() {
            true_grid.push(
                row.iter()
                    .map(|s| {
                        let mut new_value = s.original_value + modifyer;
                        if new_value > 9 {
                            new_value -= 9;
                        }

                        Point::new(s.x, s.y + grid_len, new_value)
                    })
                    .collect::<Vec<Point>>(),
            );
        }
    }
    // true_grid.print(1);

    part1(&true_grid);
}
