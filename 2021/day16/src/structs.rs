#[derive(Debug, Clone)]
pub struct Package {
    version: u32,
    type_id: u32,
    litteral_value: Option<u64>,
    sub_packages: Option<Box<Vec<Self>>>,
}
impl Package {
    pub fn new(binary: &str) -> (Self, String) {
        println!("{}", binary);

        let version = u32::from_str_radix(&binary[0..3], 2).unwrap();
        let type_id = u32::from_str_radix(&binary[3..6], 2).unwrap();

        // println!("Version: {}", version);
        // println!("Type: {}", type_id);

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
                    litteral_value: Some(u64::from_str_radix(&value, 2).unwrap()), 
                    sub_packages: None
                }, overflow)
            }
            _ => {
                let length_type_id = binary.chars().nth(6).unwrap();
                let overflow: String;

                let sub_packages = match length_type_id {
                    '0' => {
                        let packages_length = usize::from_str_radix(&binary[7..22], 2).unwrap();
                        let mut packgage_bytes = binary[22..(22+packages_length)].to_owned();
                        overflow = binary[(22+packages_length)..].to_owned();

                        let mut sub_packages = Vec::new();
                        while packgage_bytes.contains('1') {
                            let (sub_package, overflow) = Self::new(&packgage_bytes);
                            packgage_bytes = overflow;
                            sub_packages.push(sub_package);
                        }
                        Some(Box::new(sub_packages))
                    }
                    '1' => {
                        let number_packages = usize::from_str_radix(&binary[7..18], 2).unwrap();
                        let mut remaining_bits = binary[18..].to_owned();

                        let mut sub_packages = Vec::with_capacity(number_packages);
                        for _ in 0..number_packages {
                            let (sub_package, extra) = Self::new(&remaining_bits);
                            remaining_bits = extra;
                            sub_packages.push(sub_package);
                        }
                        overflow = remaining_bits;
                        Some(Box::new(sub_packages))
                    }
                    _ => panic!()
                };

                (Self {version, type_id, litteral_value: None, sub_packages}, overflow)
            }
        }
    }
    pub fn sum_versions(&self) -> u32 {
        let mut sum = self.version;

        if let Some(sub_packages) = self.sub_packages.clone() {
            for package in *sub_packages {
                sum += package.sum_versions();
            }
        }
        sum
    }
    pub fn value(&self) -> u64 {
        let sub_packages = self.sub_packages.clone();
        match self.type_id {
            0 => {
                // Sum
                let mut sum = 0;
                for package in *sub_packages.unwrap() {
                    sum += package.value();
                }
                sum
            }
            1 => {
                // Product
                let mut product = 1;
                for package in *sub_packages.unwrap() {
                    product *= package.value();
                }
                product
            }
            2 => {
                let mut min = u64::MAX;
                for package in *sub_packages.unwrap() {
                    let value = package.value();
                    if value < min {
                        min = value;
                    }
                }
                min
            }
            3 => {
                let mut max = 0;
                for package in *sub_packages.unwrap() {
                    let value = package.value();
                    if value > max {
                        max = value;
                    }
                }
                max
            }
            4 => {
                self.litteral_value.unwrap()
            }
            5 => {
                let sub_packages = sub_packages.unwrap();
                let sub1 = sub_packages[0].clone().value();
                let sub2 = sub_packages[1].clone().value();

                if sub1 > sub2 {
                    1
                }
                else {
                    0
                }
            }
            6 => {
                let sub_packages = sub_packages.unwrap();
                let sub1 = sub_packages[0].clone().value();
                let sub2 = sub_packages[1].clone().value();

                if sub1 > sub2 {
                    0
                }
                else {
                    1
                }
            }
            7 => {
                let sub_packages = sub_packages.unwrap();
                let sub1 = sub_packages[0].clone().value();
                let sub2 = sub_packages[1].clone().value();

                if sub1 == sub2 {
                    1
                }
                else {
                    0
                }
            }
            _ => panic!()
        }
    }
}