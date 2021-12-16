#[derive(Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub original_value: u32,
    pub value: u32,
}
impl Point {
    pub fn new(x: usize, y: usize, value: u32) -> Self {
        Self { x, y, original_value: value, value }
    }
    pub fn set_value(&mut self, neighbors: &[Self]) {
        if !neighbors.is_empty() {
            let smallest = neighbors
                .iter()
                .fold(u32::MAX, |acc, i| if i.value < acc { i.value } else { acc });
            self.value = self.original_value + smallest;
        }
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.value == other.value
    }
}
pub trait Print {
    fn print(&self, padding: usize);
}
impl Print for Vec<Vec<Point>> {
    fn print(&self, padding: usize) {
        for row in self {
            for point in row {
                let mut string = format!("{}", point.value);
                pad(&mut string, padding);
                print!("{}", string);
            }
            println!();
        }
        println!()
    }
}
fn pad(string: &mut String, to: usize) {
    for _ in 0..(to - string.len()) {
        string.push(' ');
    }
}