pub fn part1(input: &Vec<(Vec<String>, Vec<String>)>) {
    let mut sum = 0;
    for (_, data) in input.clone() {
        for code in data {
            if code.len() == 2 || code.len() == 3 || code.len() == 4|| code.len() == 7 {
                sum += 1;
            }
        }
    }
    println!("Part 1: {}", sum);
}