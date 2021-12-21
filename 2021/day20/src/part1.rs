pub fn part(image_enhancer: &Vec<char>, input_image: &[Vec<char>], recursion_limit: usize, part_number: u32) -> String {
    let mut processed_image = vec![vec!['.'; input_image[0].len() - 2]; input_image.len() - 2];
    let mut counter = 0;

    for i in 1..(input_image.len() - 1) {
        for j in 1..(input_image[0].len() - 1) {
            let mut binary_string = String::new();

            for k in (i - 1)..=(i + 1) {
                for l in (j - 1)..=(j + 1) {
                    match input_image[k][l] {
                        '#' => binary_string.push('1'),
                        '.' => binary_string.push('0'),
                        _ => panic!("Invalid")
                    }
                }
            }

            let index = usize::from_str_radix(&binary_string, 2).unwrap();
            processed_image[i - 1][j - 1] = image_enhancer[index];
            if image_enhancer[index] == '#' {
                counter += 1;
            }
        }
    }

    for row in &processed_image {
        for element in row {
            print!("{}", element);
        }
        println!()
    }
    println!();

    if recursion_limit > 0 {
        part(image_enhancer, &processed_image, recursion_limit - 1, part_number)
    }
    else {
        format!("Part {}: {}", part_number, counter)
    }
}