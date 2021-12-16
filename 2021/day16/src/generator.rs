fn value(input: u32) -> String {
    let version: u32 = (rand::random::<f32>() * 7.0).ceil() as u32;
    let mut version_binary = format!("{:b}", version);
    version_binary = format!("{}{}", zeroes((3 -version_binary.len() % 3) % 3),version_binary);

    let type_id = 4;
    let mut type_binary = format!("{:b}", type_id);
    type_binary = format!("{}{}", zeroes((3 -type_binary.len() % 3) % 3),type_binary);

    let mut values = format!("{:b}", input);
    values = format!("{}{}", zeroes((4 - values.len() % 4) % 4), values);

    let mut out = format!("{}{}", version_binary, type_binary);

    for i in (0..values.len()).step_by(4) {
        out.push_str(&format!("1{}", &values[i..(i + 4)]));
    }

    let len = out.len();
    out.replace_range((len-5)..(len-4), "0");

    out
}

fn operator(type_id: u32, values: Vec<String>) -> String {
    let version: u32 = (rand::random::<f32>() * 7.0).ceil() as u32;
    let mut version_binary = format!("{:b}", version);
    version_binary = format!("{}{}", zeroes((3 -version_binary.len() % 3) % 3),version_binary);

    let mut type_binary = format!("{:b}", type_id);
    type_binary = format!("{}{}", zeroes((3 -type_binary.len() % 3) % 3),type_binary);

    let length_type_id: char;
    let mut length: String;

    if rand::random::<bool>() {
        length_type_id = '1';
        let lenght_value = values.len();
        length = format!("{:b}", lenght_value);
        length = format!("{}{}", zeroes((11 - length.len() % 11) % 11), length);
    }
    else {
        length_type_id = '0';
        let lenght_value = values.iter().fold(0, |acc, s| acc + s.len());
        length = format!("{:b}", lenght_value);
        length = format!("{}{}", zeroes((15 - length.len() % 15) % 15), length);
    }

    let out = format!("{}{}{}{}{}", version_binary, type_binary, length_type_id, length, values.iter().fold("".to_string(), |acc, s| format!("{}{}", s, acc)));
    out
}

fn zeroes(amount: usize) -> String {
    let mut out = String::new();
    for _ in 0..amount {
        out.push('0');
    }
    out
}

fn main() {
    println!("{:X}", u64::from_str_radix(&value(36), 2).unwrap());
    // println!("{:X}", u64::from_str_radix(&operator(1, vec![value(36)]), 2).unwrap());
}