pub fn part2(input: &mut Vec<Vec<i32>>) {
    let mut counter = 0;
    for i in (0..(input.len() - 1)).step_by(3) {
        for j in 0..3 {
            let mut triangle = Vec::new();
            for k in 0..3 {
                triangle.push(input[i + k][j]);
            }
            triangle.sort();
    
            if triangle[0] + triangle[1] > triangle[2] {
                counter += 1;
            }
        }
    }
    println!("Part2: {}", counter);
}