use std::fmt::{self, Display};

pub enum Axis {
    X,
    Y,
}
impl Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::X => "X",
                Self::Y => "Y",
            }
        )
    }
}

pub struct Grid {
    grid: Vec<Vec<bool>>,
}
impl Grid {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        Self {
            grid: vec![vec![false; x_size]; y_size],
        }
    }
    pub fn mark_point(&mut self, x: usize, y: usize) {
        self.grid[y][x] = true;
    }
    pub fn fold(&mut self, axis: Axis, index: usize) {
        match axis {
            Axis::X => {
                for row in &mut self.grid {
                    let mut vec1 = row[..index].to_vec();
                    let mut vec2 = row[(index + 1)..]
                        .iter()
                        .rev()
                        .map(|v| v.clone())
                        .collect::<Vec<bool>>();
    
                    let vec_lens = ((), vec1.len(), vec2.len());
    
                    let (shorter, diff) = if vec_lens.1 < vec_lens.2 {
                        (&mut vec1, vec_lens.2 - vec_lens.1)
                    } else {
                        (&mut vec2, vec_lens.1 - vec_lens.2)
                    };
    
                    let mut longer = vec![false; diff];
                    longer.append(&mut shorter.clone());
    
                    *shorter = longer;

                    let mut out_row = Vec::with_capacity(vec_lens.1);
                    for i in 0..vec_lens.1 {
                        out_row.push(vec1[i] || vec2[i])
                    }
                    *row = out_row;
                } 
            }
            Axis::Y => {
                let mut vec1 = self.grid[..index].to_vec();
                let mut vec2 = self.grid[(index + 1)..]
                    .iter()
                    .rev()
                    .map(|v| v.clone())
                    .collect::<Vec<Vec<bool>>>();

                let vec_lens = (vec1[0].len(), vec1.len(), vec2.len());

                let (shorter, diff) = if vec_lens.1 < vec_lens.2 {
                    (&mut vec1, vec_lens.2 - vec_lens.1)
                } else {
                    (&mut vec2, vec_lens.1 - vec_lens.2)
                };

                let mut longer = vec![vec![false; vec_lens.0]; diff];
                longer.append(&mut shorter.clone());
                let true_len = longer.len();

                *shorter = longer;

                let mut out = Vec::new();

                for y in 0..true_len {
                    let mut row = Vec::new();

                    for x in 0..vec_lens.0 {
                        row.push(vec1[y][x] || vec2[y][x]);
                    }
                    out.push(row);
                }
                self.grid = out;
            }
        }
    }
    pub fn count(&self) -> u32 {
        let mut out = 0;
        for row in &self.grid {
            for value in row {
                if *value {
                    out += 1
                }
            }
        }
        out
    }
}
impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();

        for row in &self.grid {
            for value in row {
                if *value {
                    out.push('#');
                }
                else {
                    out.push(' ');
                }
            }
            out.push('\n')
        }
        write!(f, "{}", out.trim())
    }
}
