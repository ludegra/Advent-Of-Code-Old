use crate::structs::SevenSegmentDisplay;

pub fn part2(input: &Vec<(Vec<String>, Vec<String>)>) {
    let mut sum = 0;
    for (signals, data) in input.clone() {
        let mut number = String::new();
        let mut display = SevenSegmentDisplay::new();

        display.solve_segments(signals);

        for code in data {
            if let Some(value) = display.decode(code.clone()) {
                number.push(char::from_digit(value as u32, 10).unwrap());
            }
        } 
        sum += number.parse::<u32>().unwrap();
    }
    println!("Part 2: {}", sum);
}