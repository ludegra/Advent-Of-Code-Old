pub struct KeyPad {
    x: i8,
    y: i8,
}

impl KeyPad {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
    pub fn move_key(&mut self, command: char) {
        match command {
            'U' => if self.y < 1 { self.y += 1 },
            'D' => if self.y > -1 { self.y -= 1 },
            'R' => if self.x < 1 { self.x += 1 },
            'L' => if self.x > -1 { self.x -= 1 },
            _ => panic!("Invalid command")
        };
    }
    pub fn get_key(&self) -> char {
        match self.x {
            -1 => match self.y {
                1 => '1',
                0 => '4',
                -1 => '7',
                _ => panic!("Invalid"),
            },
            0 => match self.y {
                1 => '2',
                0 => '5',
                -1 => '8',
                _ => panic!("Invalid"),
            },
            1 => match self.y {
                1 => '3',
                0 => '6',
                -1 => '9',
                _ => panic!("Invalid"),
            },
            _ => panic!("Invalid"),
        }
    }
}

pub struct WeirdPad {
    x: i8,
    y: i8,
}

impl WeirdPad {
    pub fn new() -> Self {
        Self { x: -2, y: 0 }
    }
    pub fn move_key(&mut self, command: char) {
        match command {
            'U' => if self.y + self.x.abs() < 2 { self.y += 1 },
            'D' => if self.y - self.x.abs() > -2 { self.y -= 1 },
            'R' => if self.x + self.y.abs() < 2 { self.x += 1 },
            'L' => if self.x - self.y.abs() > -2 { self.x -= 1 },
            _ => panic!("Invalid command")
        };
    }
    pub fn get_key(&self) -> char {
        match self.y {
            2 => match self.x {
                0 => '1',
                _ => panic!("Invalid"),
            }
            1 => match self.x {
                -1 => '2',
                0 => '3',
                1 => '4',
                _ => panic!("Invalid"),
            }
            0 => match self.x {
                -2 => '5',
                -1 => '6',
                0 => '7',
                1 => '8',
                2 => '9',
                _ => panic!("Invalid"),
            }
            -1 => match self.x {
                -1 => 'A',
                0 => 'B',
                1 => 'C',
                _ => panic!("Invalid"),
            }
            -2 => match self.x {
                0 => 'D',
                _ => panic!("Invalid"),
            }
            _ => panic!("Invalid"),
        }
    }
}