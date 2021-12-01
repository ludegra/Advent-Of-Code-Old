use utils::input::read_input_lines;

fn main() {
    let input: Vec<i32> = read_input_lines("assets/input.txt").unwrap().iter().map(|s| s.parse().unwrap()).collect();

    let mut increase_counter = 0;
    let mut old = None;

    for i in &input {
        match old {
            Some(value) => {
                if value < *i { increase_counter += 1}
                old = Some(*i);
            }
            None => old = Some(*i),
        }
    }

    println!("Part 1: {}", increase_counter);

    increase_counter = 0;
    old = None;

    for i in 2..input.len() {
        let sum = input[i - 2] + input[i - 1] + input[i];

        match old {
            Some(value) => {
                if value < sum { increase_counter += 1}
                old = Some(sum);
            }
            None => old = Some(sum),
        }
    }

    println!("Part 2: {}", increase_counter);
}
