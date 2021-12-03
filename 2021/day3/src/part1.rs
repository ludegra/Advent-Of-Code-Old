pub fn part1(input: &Vec<Vec<char>>) {
    let mut common_chars = String::with_capacity(input[0].len());
    let mut uncommon_chars = String::with_capacity(input[0].len());

    for i in 0..input[0].len() {
        let mut one_counter = 0;
        for line in input {
            if line[i] == '1' {
                one_counter += 1;
            }
        }
        if one_counter > input.len() / 2 {
            common_chars.push('1');
            uncommon_chars.push('0');
        }
        else {
            common_chars.push('0');
            uncommon_chars.push('1');
        }
    }

    let gamma = isize::from_str_radix(&common_chars, 2).unwrap();
    let epsilon = isize::from_str_radix(&uncommon_chars, 2).unwrap();

    println!("Part 1: {}", epsilon * gamma)
}
