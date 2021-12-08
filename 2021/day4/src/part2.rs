use crate::structs::{Board, BingoOption};

pub fn part2(numbers: &Vec<i32>, boards: &Vec<Board>) {
    let mut boards = boards.clone();

    for number in numbers {
        let mut i = 0;
        while i < boards.len() {
            if let BingoOption::Bingo(value) = boards[i].mark_number(*number)  {
                if boards.len() == 1 {
                    println!("Part 2: {}", value * number);
                    return ();
                }
                else {
                    boards.remove(i);
                }
            }
            else {
                i += 1;
            }
        }
    }
}