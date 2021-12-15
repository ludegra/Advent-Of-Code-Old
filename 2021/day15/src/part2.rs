use crate::structs::Point;

pub fn part2(input: &Vec<Vec<Point>>) {
    let mut grid = Vec::with_capacity(input.len() * 5);

    for y_modifyer in 0..5 {
        for row in input {
            let mut full_row = Vec::with_capacity(input[0].len() * 5);
            for x_modifyer in 0..5 {
                full_row.append(
                    &mut row
                    .iter()
                    .map(|s| {
                            let mut value = (s.value + x_modifyer + y_modifyer) % 9;
                            if value == 0 { value = 9 }
                            
                            Point {
                            value,
                            ..*s
                        }})
                        .collect::<Vec<Point>>(),
                );
            }
            grid.push(full_row);
        }
    }
    grid[0][0].value = 0;

    for y in (0..grid.len()).rev() {
        for x in (0..grid[0].len()).rev() {
            let mut neighbors = Vec::with_capacity(2);
            if x < grid[y].len() - 1 {
                neighbors.push(grid[y][x + 1]);
            }
            if y < grid.len() - 1 {
                neighbors.push(grid[y + 1][x])
            }
            grid[y][x].set_value(&neighbors[..]);
        }
    }

    let mut has_changed = true;
    while has_changed {
        has_changed = false;
        for y in (0..grid.len()).rev() {
            for x in (0..grid[0].len()).rev() {
                let mut neighbors = Vec::with_capacity(2);
                if x < grid[y].len() - 1 {
                    neighbors.push(grid[y][x + 1]);
                }
                if x > 0 {
                    neighbors.push(grid[y][x - 1]);
                }
                if y < grid.len() - 1 {
                    neighbors.push(grid[y + 1][x]);
                }
                if y > 0 {
                    neighbors.push(grid[y - 1][x])
                }
                if y < grid.len() - 1 && x < grid[y].len() - 1 {
                    let old_value = grid[y][x].value.clone(); 
                    grid[y][x].set_value(&neighbors[..]);
                    if old_value != grid[y][x].value {
                        has_changed = true;
                    }
                }
            }
        }
    }

    println!("Part 2: {}", grid[0][0].value)
}
