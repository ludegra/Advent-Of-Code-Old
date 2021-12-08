use crate::structs::{Board, BingoOption};

pub fn part1(numbers: &Vec<i32>, boards: &Vec<Board>) {
    let mut boards = boards.clone();

    for number in numbers {
        for board in &mut boards {
            if let BingoOption::Bingo(value) = board.mark_number(*number)  {
                println!("Part 1: {}", value * number);
                return ();
            }
        }
    }
}