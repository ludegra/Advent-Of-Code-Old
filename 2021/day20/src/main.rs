use part1::part;

mod part1;
mod part2;

const MARGIN: usize = 250;

fn main() {
    let mut input = include_str!("../assets/input_ludegra.txt").trim().split("\n\n");

    let image_enhancer = input.next().unwrap().chars().collect::<Vec<_>>();

    let image_slice = input.next().unwrap().split('\n').map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut input_image = vec![vec!['.'; image_slice[0].len() + MARGIN]; image_slice.len() + MARGIN];

    for i in 0..image_slice.len() {
        for j in 0..image_slice[0].len() {
            input_image[i + MARGIN / 2][j + MARGIN / 2] = image_slice[i][j];
        }
    }

    for row in &input_image {
        for element in row {
            print!("{}", element);
        }
        println!()
    }
    println!();

    let part1_answ = part(&image_enhancer, &input_image, 1, 1);
    let part2_answ = part(&image_enhancer, &input_image, 49, 2);

    println!("{}", part1_answ);
    println!("{}", part2_answ);
}
