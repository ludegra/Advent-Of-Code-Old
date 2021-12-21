use crate::structs::Point;

pub fn part2(positions: Vec<Point>) {
    let mut biggest = 0;

    for point1 in &positions {
        for point2 in &positions {
            let diff = (point1.clone() - point2.clone()).manhattan();
            if diff > biggest {
                biggest = diff;
            }
        }
    }

    println!("Part 2: {}", biggest)
}