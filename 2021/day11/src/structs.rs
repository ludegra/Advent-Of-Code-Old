#[derive(Debug, Clone)]
pub struct Octopus<'a> {
    x: usize,
    y: usize,
    grid: *mut Vec<Vec<Self>>,
    pub energy: u32,
    has_flashed: bool,
}
impl<'a> Octopus<'a> {
    pub fn new(x: usize, y: usize, grid: *mut Vec<Vec<Self>>, energy: u32) -> Self {
        Self {
            x,
            y,
            grid,
            energy,
            has_flashed: false,
        }
    }
    pub fn increase_energy(&mut self) {
        self.energy += 1;

        // if self.energy > 9 && !self.has_flashed {
        //     self.flash();
        // }
    }
    pub fn flash(&mut self) -> Option<()> {
        if self.energy <= 9 || self.has_flashed {return None}

        let grid;
        unsafe {
            grid = self.grid.as_mut().unwrap()
        }

        let min_x = if self.x == 0 {0} else {self.x - 1};
        let min_y = if self.y == 0 {0} else {self.y - 1};
        let max_x = if self.x == grid[0].len() - 1 {grid[0].len() - 1} else {self.x + 1};
        let max_y = if self.y == grid.len() - 1 {grid.len() - 1} else {self.y + 1};

        // println!("Octopus at ({},{})", self.x, self.y);
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if !(y == self.y && x == self.x) {
                    // print!("({},{}) ", x, y);
                    grid[y][x].increase_energy()
                }
            }
        }
        // println!("\n");
        self.has_flashed = true;
        Some(())
    }
    pub fn finish_round(&mut self, flash_counter: &mut u32) {
        if self.energy > 9 {
            self.energy = 0;
            *flash_counter += 1;
        }
        self.has_flashed = false;
    }
}
