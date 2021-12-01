pub fn part1(input: &mut Vec<Vec<i32>>) {
    let mut counter = 0;
    for triangle in input {
        triangle.sort();

        if triangle[0] + triangle[1] > triangle[2] {
            counter += 1;
        }
    }

    println!("Part1: {}", counter);
}