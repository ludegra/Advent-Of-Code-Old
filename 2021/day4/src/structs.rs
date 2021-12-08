use std::ops::{Index, IndexMut};

pub enum BingoOption<T> {
    Bingo(T),
    None
}

#[derive(Clone, Copy, PartialEq)]
pub struct Board {
    r0: [(i32, bool); 5],
    r1: [(i32, bool); 5],
    r2: [(i32, bool); 5],
    r3: [(i32, bool); 5],
    r4: [(i32, bool); 5],
}

impl Board {
    pub fn new(values: &[Vec<i32>]) -> Self {
        Self {
            r0: [(values[0][0], false), (values[0][1], false), (values[0][2], false), (values[0][3], false), (values[0][4], false)],
            r1: [(values[1][0], false), (values[1][1], false), (values[1][2], false), (values[1][3], false), (values[1][4], false)],
            r2: [(values[2][0], false), (values[2][1], false), (values[2][2], false), (values[2][3], false), (values[2][4], false)],
            r3: [(values[3][0], false), (values[3][1], false), (values[3][2], false), (values[3][3], false), (values[3][4], false)],
            r4: [(values[4][0], false), (values[4][1], false), (values[4][2], false), (values[4][3], false), (values[4][4], false)],
        }
    }
    pub fn mark_number(&mut self, number: i32) -> BingoOption<i32> {
        'outer: for i in 0..5 {
            for j in 0..5 {
                if self[i][j].0 == number {
                    self[i][j].1 = true;

                    if self.check_row(i) || self.check_collumn(j) {
                        return BingoOption::Bingo(self.sum_unmarked());
                    }

                    break 'outer;
                }
            }
        }
        BingoOption::None
    }
    fn check_row(&self, index: usize) -> bool {
        for value in self[index] {
            if !value.1 {
                return false;
            }
        }
        true
    }
    fn check_collumn(&self, index: usize) -> bool {
        for i in 0..5 {
            if !self[i][index].1 {
                return false;
            }
        }
        true
    }
    fn sum_unmarked(&self) -> i32 {
        let mut sum = 0;

        for i in 0..5 {
            for value in self[i] {
                if !value.1 {
                    sum += value.0;
                }
            }
        }

        sum
    }
}

impl Index<usize> for Board {
    type Output = [(i32, bool); 5];

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r0,
            1 => &self.r1,
            2 => &self.r2,
            3 => &self.r3,
            4 => &self.r4,
            _ => panic!("Index out of bounds: index was {}, but the length was 5", index),
        }
    }
}
impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r0,
            1 => &mut self.r1,
            2 => &mut self.r2,
            3 => &mut self.r3,
            4 => &mut self.r4,
            _ => panic!("Index out of bounds: index was {}, but the length was 5", index),
        }
    }
}