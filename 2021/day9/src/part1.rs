use crate::structs::Point;

pub fn part1(input: &Vec<Vec<u32>>, low_points: &mut Vec<Point>) {
    let mut sum  = 0;

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let current = input[i][j];

            if j > 0 {
                if input[i][j-1] <= current { continue }
            }
            if j < input[0].len() - 1 {
                if input[i][j+1] <= current { continue }
            }
            if i > 0 {
                if input[i-1][j] <= current { continue }
            }
            if i < input.len() - 1 {
                if input[i+1][j] <= current { continue }
            }
            sum += current + 1;
            low_points.push(Point {
                level: current,
                x: j,
                y: i
            });
        }
    }

    println!("Part 1: {}", sum);
}