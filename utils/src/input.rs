use std::io::{self, Error, prelude::*};
use std::fs::File;

pub fn read_input(input_file: &str) -> Result<String, Error> {
    let mut file = File::open(input_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn read_input_lines(input_file: &str) -> Result<Vec<String>, Error> {
    let file = File::open(input_file)?;
    let lines = io::BufReader::new(file).lines();

    let mut out = Vec::new();
    for line in lines {
        if let Ok(line) = line {
            out.push(line);
        }
    }
    Ok(out)
}