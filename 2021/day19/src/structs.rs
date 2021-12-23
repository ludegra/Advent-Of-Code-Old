use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, Sub},
};

pub enum RotationAxis {
    X,
    Y,
    Z,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Point {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}
impl Point {
    pub fn rotated_90(&self, axis: RotationAxis) -> Self {
        let mut out = self.clone();

        match axis {
            RotationAxis::X => {
                out.y = -self.z;
                out.z = self.y;
            }
            RotationAxis::Y => {
                out.x = self.z;
                out.z = -self.x;
            }
            RotationAxis::Z => {
                out.x = -self.y;
                out.y = self.x;
            }
        }
        out
    }
    pub fn all_rotations(&self) -> Vec<Self> {
        let mut out = vec![
            self.clone(),
            self.rotated_90(RotationAxis::X),
            self.rotated_90(RotationAxis::X).rotated_90(RotationAxis::X),
            self.rotated_90(RotationAxis::X)
                .rotated_90(RotationAxis::X)
                .rotated_90(RotationAxis::X),
        ];

        let mut y_rotation = Vec::new();

        y_rotation.append(
            &mut out
                .iter()
                .map(|s| s.rotated_90(RotationAxis::Y))
                .collect::<Vec<_>>(),
        );
        y_rotation.append(
            &mut out
                .iter()
                .map(|s| s.rotated_90(RotationAxis::Y).rotated_90(RotationAxis::Y))
                .collect::<Vec<_>>(),
        );
        y_rotation.append(
            &mut out
                .iter()
                .map(|s| {
                    s.rotated_90(RotationAxis::Y)
                        .rotated_90(RotationAxis::Y)
                        .rotated_90(RotationAxis::Y)
                })
                .collect::<Vec<_>>(),
        );
        y_rotation.append(
            &mut out
                .iter()
                .map(|s| s.rotated_90(RotationAxis::Z))
                .collect::<Vec<_>>(),
        );
        y_rotation.append(
            &mut out
                .iter()
                .map(|s| {
                    s.rotated_90(RotationAxis::Z)
                        .rotated_90(RotationAxis::Z)
                        .rotated_90(RotationAxis::Z)
                })
                .collect::<Vec<_>>(),
        );

        out.append(&mut y_rotation);

        out
    }
    pub fn manhattan(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scanner {
    pub index: u32,

    beacons: Vec<Point>,
}
impl Scanner {
    pub fn new(index: u32, beacons: &[Point]) -> Self {
        let mut beacons = beacons.to_vec();
        beacons.sort();
        Self { index, beacons }
    }
    pub fn number_beacons(&self) -> usize {
        self.beacons.len()
    }
    pub fn all_rotations(&self) -> Vec<Self> {
        let beacons = self
            .beacons
            .iter()
            .map(|s| s.all_rotations())
            .collect::<Vec<_>>();

        let mut out = Vec::new();
        for i in 0..24 {
            let mut beacons = beacons.iter().fold(Vec::new(), |mut acc, s| {
                acc.push(s[i].clone());
                acc
            });
            beacons.sort();

            out.push(Self {
                index: self.index,
                beacons,
            });
        }
        out
    }
    pub fn compare_points(&mut self, other: &mut Scanner) -> Option<Point> {
        let mut diffs = HashMap::new();
        for beacon1 in &self.beacons {
            for beacon2 in &other.beacons {
                let diff = beacon1.clone() - beacon2.clone();
                let entry = diffs.entry(diff).or_insert(0);
                *entry += 1;
            }
        }

        let mut diffs = diffs.iter().collect::<Vec<_>>();
        diffs.sort_by(|a, b| b.1.cmp(a.1));

        if *diffs[0].1 >= 12 {
            let diff = diffs[0].0.clone();

            for beacon in &other.beacons {
                let absolute_beacon = beacon.clone() + diff.clone();
                if !self.beacons.contains(&absolute_beacon) {
                    self.beacons.push(absolute_beacon);
                }
            }
            return Some(diff);
        }
        None
    }
    pub fn _print_beacons(&self) {
        let mut beacons = self.beacons.clone();
        beacons.sort();

        for beacon in beacons {
            println!("{}", beacon);
        }
    }
}
