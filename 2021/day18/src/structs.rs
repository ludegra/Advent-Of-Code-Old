use std::{
    fmt::{self, Display},
    ops::{Add, AddAssign},
};

enum Side {
    Right,
    Left,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PairComponent {
    Value(u32),
    Pair(Box<PairComponent>, Box<PairComponent>),
}
impl PairComponent {
    pub fn check_explosion(&mut self, level: u32) -> (Option<u32>, Option<u32>) {
        match self {
            Self::Pair(left, right) => {
                if level >= 4 {
                    let mut out = (None, None);
                    if let Self::Value(value) = **left {
                        out.0 = Some(value);
                    }
                    if let Self::Value(value) = **right {
                        out.1 = Some(value);
                    }
                    // println!("\n    Explosion: {}:", self);
                    *self = PairComponent::Value(0);
                    return out;
                } else {
                    let (out_left, sub_right) = left.check_explosion(level + 1);
                    if let Some(sub_right) = sub_right {
                        right.add_value(sub_right, Side::Left);
                    }
                    let (sub_left, out_right) = right.check_explosion(level + 1);
                    if let Some(sub_left) = sub_left {
                        left.add_value(sub_left, Side::Right);
                    }
                    return (out_left, out_right);
                }
            }
            Self::Value(_) => {
                return (None, None);
            }
        }
    }
    fn add_value(&mut self, value: u32, side: Side) {
        match self {
            Self::Pair(left, right) => match side {
                Side::Left => left.add_value(value, side),
                Side::Right => right.add_value(value, side),
            },
            Self::Value(old_value) => {
                // println!("        Adding: {} + {} = {}", old_value, value, *old_value + value);
                *old_value += value;
            },
        }
    }
    pub fn check_split(&mut self) -> bool {
        match self {
            Self::Pair(left, right) => {
                if left.check_split() {
                    return true;
                }
                else if right.check_split() {
                    return true;
                }
            }
            Self::Value(value) => {
                if *value >= 10 {
                    let half = *value / 2;
                    let remainder = *value % 2;

                    // let value = *value;

                    *self = PairComponent::Pair(
                        Box::new(PairComponent::Value(half)),
                        Box::new(PairComponent::Value(half + remainder)),
                    );
                    // println!("    Split: {} -> {}", value, self);
                    return true;
                }
            }
        }
        false
    }
    pub fn reduce(&mut self) {
        let old = self.clone();
        self.check_explosion(0);
        if *self == old {
            // println!();
            self.check_split();
        }
        // print!("\n    {}", self);
        if *self != old {
            self.reduce()
        }
    }
    pub fn magnitude(&self) -> u32 {
        match self {
            PairComponent::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
            PairComponent::Value(value) => *value
        }
    }
}

impl Add for PairComponent {
    type Output = PairComponent;

    fn add(self, other: PairComponent) -> Self::Output {
        let mut result = Self::Pair(Box::new(self), Box::new(other));
        // println!("\n{}:", result);
        result.reduce();
        // println!();
        result
    }
}
impl AddAssign for PairComponent {
    fn add_assign(&mut self, other: PairComponent) {
        *self = self.clone() + other
    }
}

impl Display for PairComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PairComponent::Pair(left, right) => write!(f, "[{},{}]", *left, *right),
            PairComponent::Value(value) => write!(f, "{}", value),
        }
    }
}
