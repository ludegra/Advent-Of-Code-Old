pub struct Package2 {
    version: u32,
    type_id: u32,
    sub_packages: Option<Box<Vec<Package2>>>,
    value: u64,
}
impl Package2 {
    pub fn new(binary: &str) -> (Self, String) {
        let version = u32::from_str_radix(&binary[0..3], 2).unwrap();
        let type_id = u32::from_str_radix(&binary[3..6], 2).unwrap();
        
        match type_id {
            4 => {
                let mut overflow = String::new();
                let mut value = String::new();
                for i in (6..).step_by(5) {
                    let label = binary.chars().nth(i).unwrap();

                    value.push_str(&binary[(i+1)..(i+5)]);

                    if label == '0' {
                        overflow = binary[(i+5)..].to_owned();
                        break;
                    }
                }
                (Self {
                    version,
                    type_id,
                    value: u64::from_str_radix(&value, 2).unwrap(), 
                    sub_packages: None
                }, overflow)
            }
            _ => {
                let len_type = binary.chars().nth(6).unwrap();
                let value: u64;
                let mut children = Vec::new();
                let mut sub_packages_bits: String;
                let mut overflow: String;

                if len_type == '0' {
                    let len = u64::from_str_radix(&binary[7..(7 + 15)], 2).unwrap();
                    sub_packages_bits = binary[(7 + 15)..(7 + 15 + len as usize)].to_owned();

                    while sub_packages_bits.contains("1") {
                        let (child, overflow) = Self::new(&sub_packages_bits);
                        sub_packages_bits = overflow;
                        children.push(child);
                    }

                    overflow = binary[(7 + 15 + len as usize)..].to_owned();
                }
                else {
                    let len = u64::from_str_radix(&binary[7..(7 + 11)], 2).unwrap();
                    overflow = binary[(7 + 11)..].to_owned();

                    for _ in 0..len {
                        let (child, extra) = Self::new(&overflow);
                        children.push(child);
                        overflow = extra;
                    }
                }
                value = Self::calculate(type_id, &children);

                (Self {
                    version,
                    type_id,
                    value,
                    sub_packages: Some(Box::new(children)),
                }, overflow)
            }
        }
    }
    fn calculate(type_id: u32, sub_packages: &Vec<Package2>) -> u64 {
        match type_id {
            0 => sub_packages.iter().fold(0, |acc, s| acc + s.value),
            1 => sub_packages.iter().fold(1, |acc, s| acc * s.value),
            2 => sub_packages.iter().fold(u64::MAX, |acc, s| if acc < s.value {acc} else {s.value}),
            3 => sub_packages.iter().fold(0, |acc, s| if acc > s.value {acc} else {s.value}),
            5 => (sub_packages[0].value > sub_packages[1].value) as u64,
            6 => (sub_packages[0].value < sub_packages[1].value) as u64,
            7 => (sub_packages[0].value == sub_packages[1].value) as u64,
            _ => panic!()
        }
    }
    pub fn value(&self) -> u64 {
        self.value
    }
}
