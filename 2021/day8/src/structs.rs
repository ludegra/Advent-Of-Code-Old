use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct SevenSegmentDisplay {
    a: Vec<char>,
    b: Vec<char>,
    c: Vec<char>,
    d: Vec<char>,
    e: Vec<char>,
    f: Vec<char>,
    g: Vec<char>,

    zero: Option<String>,
    one: Option<String>,
    two: Option<String>,
    three: Option<String>,
    four: Option<String>,
    five: Option<String>,
    six: Option<String>,
    seven: Option<String>,
    eight: Option<String>,
    nine: Option<String>,
}
impl SevenSegmentDisplay {

    pub fn new() -> Self {
        Self {
            a: vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'],
            b: vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'],
            c: vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'],
            d: vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'],
            e: vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'],
            f: vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'],
            g: vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'],

            zero: None,
            one: None,
            two: None,
            three: None,
            four: None,
            five: None,
            six: None,
            seven: None,
            eight: None,
            nine: None,
        }
    }
    pub fn solve_segments(&mut self, signals: Vec<String>) {
        for signal in signals {
            let mut signal = signal.chars().collect::<Vec<char>>();
            signal.sort();
            let signal = signal.iter().collect::<String>();

            match signal.len() {
                2 => {
                    self.one = Some(signal.clone());

                    self['c'] = Vec::new();
                    self['f'] = Vec::new();
                    for wire in signal.chars() {
                        self['c'].push(wire);
                        self['f'].push(wire);
                    }
                }
                3 => {
                    self.seven = Some(signal.clone());

                    let mut left = signal.clone();

                    for occupied in self.one.clone().unwrap().chars() {
                        left = left.replace(occupied, "");
                    }

                    self['a'] = Vec::new();
                    for wire in left.chars() {
                        self['a'].push(wire);
                    }
                }
                4 => {
                    self.four = Some(signal.clone());

                    let mut left = signal.clone();

                    for occupied in self.one.clone().unwrap().chars() {
                        left = left.replace(occupied, "");
                    }

                    self['b'] = Vec::new();
                    self['d'] = Vec::new();
                    for wire in left.chars() {
                        self['b'].push(wire);
                        self['d'].push(wire);
                    }
                }
                5 => {
                    let mut remaining = "abcdefg".to_string();
                    for wire in signal.chars() {
                        remaining = remaining.replace(wire, "");
                    }

                    println!("{}", remaining);
                    println!("{:?}\n", self);

                    for wire in remaining.chars() {
                        if self['c'].contains(&wire) {
                            // fixa logik 2, 3, 5
                            if !self['b'].contains(&wire) {
                                self.five = Some(signal.clone());
                            }
                            else {
                                self.two = Some(signal.clone());
                            }

                            self['e'] = vec![remaining.replace(wire, "").chars().next().unwrap()];

                            if self['c'].len() > 1 {
                                self['c'] = vec![wire];
                                self['f'] = vec![self.one.clone().unwrap().replace(wire, "").chars().next().unwrap()];
                            }
                        }
                        else if self['f'].contains(&wire) {
                            if !self['b'].contains(&wire) {
                                self.five = Some(signal.clone());
                            }
                            else {
                                self.two = Some(signal.clone());
                            }

                            self['b'] = vec![remaining.replace(wire, "").chars().next().unwrap()];

                            if self['f'].len() > 1 {
                                self['f'] = vec![wire];
                                self['c'] = vec![self.one.clone().unwrap().replace(wire, "").chars().next().unwrap()];
                            }
                        }
                        else {

                        }
                    }
                },
                6 => {
                    let mut remaining = "abcdefg".to_string();

                    for wire in signal.chars() {
                        remaining = remaining.replace(wire, "");
                    }

                    let remaining = remaining.chars().next().unwrap();

                    if self['d'].contains(&remaining) {
                        self.zero = Some(signal.clone());
                        self['d'] = vec![remaining];
                    }
                    else if self['c'].contains(&remaining) {
                        self.six = Some(signal.clone());
                    }
                    else {
                        self.nine = Some(signal.clone());
                    }
                }
                7 => {
                    self.eight = Some(signal.clone());

                    let mut remaining = "abcdefg".to_string();

                    for segment in "abcdef".chars() {
                        remaining = remaining.replace(self[segment][0], "");
                    }

                    self['g'] = vec![remaining.chars().next().unwrap()];
                }
                _ => (),
            }
        }
    }
    pub fn decode(&self, input: String) -> Option<usize> {
        let mut sorter = input.chars().collect::<Vec<char>>();
        sorter.sort();
        let sorted = sorter.iter().collect::<String>();

        for i in 0..=9 {
            if self[i].clone().unwrap() == sorted {
                return Some(i);
            }
        }
        None
    }
}
impl Index<char> for SevenSegmentDisplay {
    type Output = Vec<char>;

    fn index(&self, index: char) -> &Self::Output {
        match index {
            'a' => &self.a,
            'b' => &self.b,
            'c' => &self.c,
            'd' => &self.d,
            'e' => &self.e,
            'f' => &self.f,
            'g' => &self.g,
            _ => panic!("Index out of bounds"),
        }
    }
}
impl IndexMut<char> for SevenSegmentDisplay {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        match index {
            'a' => &mut self.a,
            'b' => &mut self.b,
            'c' => &mut self.c,
            'd' => &mut self.d,
            'e' => &mut self.e,
            'f' => &mut self.f,
            'g' => &mut self.g,
            _ => panic!("Index out of bounds"),
        }
    }
}
impl Index<usize> for SevenSegmentDisplay {
    type Output = Option<String>;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.zero,
            1 => &self.one,
            2 => &self.two,
            3 => &self.three,
            4 => &self.four,
            5 => &self.five,
            6 => &self.six,
            7 => &self.seven,
            8 => &self.eight,
            9 => &self.nine,
            _ => panic!("Index out of bounds"),
        }
    }
}
