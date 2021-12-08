use crate::structs::SevenSegmentDisplay;

pub fn part1(input: &Vec<(Vec<String>, Vec<String>)>) {
    for (signals, data) in input.clone() {
        let mut display = SevenSegmentDisplay::new();

        display.solve_segments(signals);
        println!("{:?}", display);

        for code in data {
            if let Some(number) = display.decode(code.clone()) {
                println!("{} -> {}", code, number);
            }
            else {
                println!("{} -> None", code)
            }
        }
        println!("");
    }
}