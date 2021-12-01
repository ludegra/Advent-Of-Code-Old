use std::fs::File;
use std::io::prelude::*;

const INPUT_PATH: &'static str = "./assets/input.txt";

fn main() {
    let mut file = File::open(INPUT_PATH).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let squares = content.split("\r\n\r\n").collect::<Vec<&str>>();

    println!("{:?}", squares);
}