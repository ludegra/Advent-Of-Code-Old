use std::thread;

pub fn part2(input: &Vec<Vec<char>>) {
    let mut o2 = input.clone();
    let mut co2 = input.clone();

    let o2_thread = thread::spawn(move || {
        for i in 0..o2[0].len() {
            if o2.len() == 1 { break }

            o2.sort_by(|a,b| a[i].cmp(&b[i]));
            // println!("i: {}, o2: {:?}", i, o2);
            let mut counter = 0;
            for j in 0..o2.len() {
                if o2[j][i] == '1' {
                    counter += 1;
                }
            }
            // println!("counter: {}, half: {}\n", counter, counter >= o2.len() / 2 + o2.len() % 2);
            if counter >= o2.len() / 2 + o2.len() % 2 {
                o2 = o2[(o2.len() - counter)..].to_vec();
            }
            else {
                o2 = if counter > 0 { o2[..counter].to_vec() } else { o2[..].to_vec() }
            }
        }
        i32::from_str_radix(&o2[0].iter().collect::<String>(), 2).unwrap()
    });

    let co2_thread = thread::spawn(move || {
        for i in 0..co2[0].len() {
            if co2.len() == 1 { break }

            co2.sort_by(|a,b| a[i].cmp(&b[i]));
            println!("i: {}, CO2: {:?}", i, co2);
            let mut counter = 0;
            for j in 0..co2.len() {
                if co2[j][i] == '1' {
                    counter += 1;
                }
            }
            println!("counter: {}, half: {}\n", counter, counter >= co2.len() / 2 + co2.len() % 2);
            if counter >= co2.len() / 2 + co2.len() % 2 {
                co2 = if counter > 0 { co2[..(co2.len() - counter - 1)].to_vec() } else { co2[..].to_vec() }
            }
            else {
                co2 = co2[(co2.len() - counter)..].to_vec();
            }
        }
        i32::from_str_radix(&co2[0].iter().collect::<String>(), 2).unwrap()
    });

    let o2_value = o2_thread.join().unwrap();
    let co2_value = co2_thread.join().unwrap();

    println!("O2 value: {}", o2_value);
    println!("CO2 value: {}", co2_value);

    println!("Part 2: {}", o2_value * co2_value);
}