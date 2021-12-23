pub fn part1(input: &Vec<(bool, (isize, isize), (isize, isize), (isize, isize))>) {
    let mut grid = [[[false; 101]; 101]; 101];
    let mut counter = 0;

    for (powered, x_range, y_range, z_range) in input {
        for x in x_range.0..=x_range.1 {
            let x = x + 50;
            if x < 0 { continue }
            else if x > 100 { break }

            for y in y_range.0..=y_range.1 {
                let y = y + 50;
                if y < 0 { continue }
                else if y > 100 { break }

                for z in z_range.0..=z_range.1 {
                    let z = z + 50;
                    if z < 0 { continue }
                    else if z > 100 { break }

                    if grid[x as usize][y as usize][z as usize] != *powered {
                        if *powered { counter += 1 } else { counter -= 1 }
                    }
                    grid[x as usize][y as usize][z as usize] = *powered;
                }
            }
        }
    }

    println!("Part 1: {}", counter);
}