pub enum TurningDirection {
    Right,
    Left
}

impl TurningDirection {
    pub fn match_char(character: char) -> Self {
        match character {
            'L' => TurningDirection::Left,
            'R' => TurningDirection::Right,
            _ => panic!("Invalid char"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum FacingDirection {
    Right,
    Left,
    Up,
    Down,
}

impl FacingDirection {
    pub fn turn(&mut self, direction: TurningDirection) -> Self {
        let as_array = [Self::Up, Self::Right, Self::Down, Self::Left];
        let index: i8 = match self {
            Self::Up => 0,
            Self::Right => 1,
            Self::Down => 2,
            Self::Left => 3,
        };
        match direction {
            TurningDirection::Right => {
                let index = (index + 1) % 4;
                as_array[index as usize]
            },
            TurningDirection::Left => {
                let mut index = (index - 1) % 4;
                if index < 0 { index += 4 }
                as_array[index as usize]
            },
        }
    }
}

pub struct Coordinate {
    x: i32,
    y: i32
}

impl Coordinate {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
    pub fn walk(&mut self, direction: FacingDirection, length: i32) {
        match direction {
            FacingDirection::Up => self.y += length,
            FacingDirection::Down => self.y -= length,
            FacingDirection::Right => self.x += length,
            FacingDirection::Left => self.x -= length,
        };
    }
    pub fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
    pub fn get_coords(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}