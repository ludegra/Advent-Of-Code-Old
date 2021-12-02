pub struct Coordinate {
    horizontal: i32,
    depth: i32,
}

impl Coordinate {
    pub fn new() -> Self {
        Self { horizontal: 0, depth: 0 }
    }
    pub fn forwards(&mut self, steps: i32) {
        self.horizontal += steps;
    }
    pub fn up(&mut self, steps: i32) {
        self.depth -= steps;
    }
    pub fn down(&mut self, steps: i32) {
        self.depth += steps;
    }
    pub fn result(&self) -> i32 {
        self.horizontal * self.depth
    }
}

pub struct AimCoords {
    horizontal: i32,
    depth: i32,
    aim: i32
}
impl AimCoords {
    pub fn new() -> Self {
        Self { horizontal: 0, depth: 0, aim: 0 }
    }
    pub fn down(&mut self, steps: i32) {
        self.aim += steps;
    }
    pub fn up(&mut self, steps: i32) {
        self.aim -= steps;
    }
    pub fn forwards(&mut self, steps: i32) {
        self.horizontal += steps;
        self.depth += self.aim * steps;
    }
    pub fn result(&self) -> i32 {
        self.horizontal * self.depth
    }
}