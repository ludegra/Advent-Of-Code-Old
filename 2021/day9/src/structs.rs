use colored::Colorize;

#[derive(Clone, Debug)]
pub struct Point {
    pub level: u32,
    pub x: usize,
    pub y: usize,
}
impl Point {
    pub fn find_neighbors(&self, grid: &Vec<Vec<u32>>) -> Vec<Point> {
        let mut neighbors = Vec::new();
        if self.y > 0 {
            neighbors.push(Point {
                level: grid[self.y - 1][self.x],
                x: self.x,
                y: self.y - 1,
            });
        }
        if self.y < grid.len() - 1 {
            neighbors.push(Point {
                level: grid[self.y + 1][self.x],
                x: self.x,
                y: self.y + 1,
            });
        }
        if self.x > 0 {
            neighbors.push(Point {
                level: grid[self.y][self.x - 1],
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.x < grid[0].len() - 1 {
            neighbors.push(Point {
                level: grid[self.y][self.x + 1],
                x: self.x + 1,
                y: self.y,
            });
        }

        neighbors
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub enum CouldExpand {
    Success,
    Faliure(usize)
}

#[derive(Debug)]
pub struct Basin<'a> {
    level: u32,
    point_vec: Vec<Point>,
    grid: &'a Vec<Vec<u32>>,
    size: usize,
}
impl<'a> Basin<'a> {
    pub fn new(starting_point: Point, grid: &'a Vec<Vec<u32>>) -> Self {
        let level = starting_point.level;
        Self {
            level,
            point_vec: vec![starting_point],
            grid,
            size: 1,
        }
    }
    pub fn increase_level(&mut self, last_increase: Option<u32>) -> CouldExpand {        
        let mut has_changed = true;
        let mut has_expanded = false;
        let level = if let Some(level) = last_increase { level } else {self.level};
        let mut current_points = self.point_vec.clone();
        let mut added_points = Vec::new();

        while has_changed {

            has_changed = false;
            let mut new_points = Vec::new();

            for points in current_points {
                let neighbors = points.find_neighbors(self.grid);

                for neighbor in neighbors {
                    if neighbor.level == self.level + 1 && !self.point_vec.contains(&neighbor) && !new_points.contains(&neighbor) {
                        new_points.push(neighbor);
                        has_changed = true;
                        has_expanded = true;
                    }
                    else if neighbor.level < self.level + 1 && !self.point_vec.contains(&neighbor) {
                        return CouldExpand::Faliure(self.size);
                    }
                }
            }
            current_points = new_points.clone();
            self.point_vec.append(&mut new_points.clone());
            added_points.append(&mut new_points);
        }
        self.size += added_points.len();
        self.level += 1;
        
        // self.print();

        if has_expanded { CouldExpand::Success } else { self.increase_level(Some(level)) }
    }
    pub fn print(&self) {
        println!("Level: {}", self.level);
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                if self.point_vec.contains(&Point {
                    level: self.grid[i][j],
                    x: j,
                    y: i
                }) {
                    print!("{}", format!("{}", self.grid[i][j]).blue());
                }
                else {
                    print!("{}", self.grid[i][j]);
                }
            }
            println!("")
        }
        println!("");
    }
}