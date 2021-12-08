use std::fmt::{Display, Formatter, self};

pub struct Grid {
    grid: [[u32; 1000]; 1000],
    number_2_or_more: u32,
}
impl Grid {
    pub fn new() -> Self {
        Self {
            grid: [[0; 1000]; 1000],
            number_2_or_more: 0,
        }
    }
    pub fn mark_straight_line(&mut self, start_coords: (usize, usize), end_coords: (usize, usize)) {
        if start_coords.0 == end_coords.0 {
            let biggest = if start_coords.1 > end_coords.1 { start_coords.1 } else { end_coords.1 };
            let smallest = if start_coords.1 < end_coords.1 { start_coords.1 } else { end_coords.1 };

            for i in smallest..=biggest {
                self.mark_point(i, start_coords.0);
            }
        }
        else if start_coords.1 == end_coords.1 {
            let biggest = if start_coords.0 > end_coords.0 { start_coords.0 } else { end_coords.0 };
            let smallest = if start_coords.0 < end_coords.0 { start_coords.0 } else { end_coords.0 };

            for i in smallest..=biggest {
                self.mark_point(start_coords.1, i);
            }
        }
        else {
            return ();
        }
    }
    pub fn mark_line(&mut self, start_coords: (usize, usize), end_coords: (usize, usize)) {
        if start_coords.0 == end_coords.0 {
            let biggest = if start_coords.1 > end_coords.1 { start_coords.1 } else { end_coords.1 };
            let smallest = if start_coords.1 < end_coords.1 { start_coords.1 } else { end_coords.1 };

            for i in smallest..=biggest {
                self.mark_point(start_coords.0, i);
            }
        }
        else if start_coords.1 == end_coords.1 {
            let biggest = if start_coords.0 > end_coords.0 { start_coords.0 } else { end_coords.0 };
            let smallest = if start_coords.0 < end_coords.0 { start_coords.0 } else { end_coords.0 };

            for i in smallest..=biggest {
                self.mark_point(i, start_coords.1);
            }
        }
        else if ((start_coords.1 as isize - end_coords.1 as isize) / (start_coords.0 as isize - end_coords.0 as isize)).abs() == 1 {
            let f = |x: usize| {
                let x = x as isize;
                let k = (start_coords.1 as isize - end_coords.1 as isize) / (start_coords.0 as isize - end_coords.0 as isize);
                let m = start_coords.1 as isize - k * start_coords.0 as isize;

                (k * x + m) as usize
            };

            let biggest = if start_coords.0 > end_coords.0 { start_coords.0 } else { end_coords.0 };
            let smallest = if start_coords.0 < end_coords.0 { start_coords.0 } else { end_coords.0 };

            for x in smallest..=biggest {
                self.mark_point(x, f(x));
            }
        }
        else {
            return ();
        }
    }
    fn mark_point(&mut self, x: usize, y: usize) {
        if self.grid[x][y] == 1 {
            self.number_2_or_more += 1;
        }
        self.grid[x][y] += 1;
    }
    pub fn number_2_or_more(&self) -> u32 {
        self.number_2_or_more
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut out_string = String::new();

        for y in 0..10 {
            for x in 0..10 {
                let point = self.grid[x][y];
                if point > 0 {
                    out_string.push_str(&point.to_string());
                }
                else {
                    out_string.push('.');
                }
            }
            out_string.push('\n');
        }

        write!(f, "{}", out_string)
    }
}