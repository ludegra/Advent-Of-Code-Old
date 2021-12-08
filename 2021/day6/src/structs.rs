use std::collections::HashMap;

#[derive(Debug)]
pub struct Fish {
    timer: i32,
    time_left: i32,
}
impl Fish {
    pub fn new(timer: i32, time_left: i32) -> Self {
        Self { timer, time_left }
    }
    pub fn count_children(&self, checker: &mut HashMap<i32, Option<u64>>) -> u64 {
        let first_child = self.time_left - (self.timer + 1);
        let entry = checker.entry(first_child).or_insert(None);

        match entry {
            Some(entry) => *entry,
            None => {
                let mut sum = 1;
        
                if first_child >= 0  {
                    for i in (0..=(first_child)).step_by(7) {
                        let child = Fish::new(8, first_child - i);
                        sum += child.count_children(checker);
                    }
                }
                let entry = checker.entry(first_child).or_insert(None);
                *entry = Some(sum);
                sum
            }
        }
    }
}
