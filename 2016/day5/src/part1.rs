use std::sync::mpsc::Receiver;

pub fn part1(reciever: &Receiver<String>) -> String {
    let mut output = String::new();
    let mut counter = 0;

    for recieved in reciever {
        output.push(recieved.chars().nth(5).unwrap());
        counter += 1;

        if counter == 8 {
            break;
        }
    }

    output
}