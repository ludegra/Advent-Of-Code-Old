use std::thread;

pub fn part2(input: &Vec<Vec<char>>) {
    let mut o2 = input.clone();
    let mut co2 = input.clone();

    let o2_thread = thread::spawn(move || {
        for i in 0..o2[0].len() {
            if o2.len() == 1 { break }

            o2.sort_by(|a,b| a[i].cmp(&b[i]));

            for j in 0..o2.len() {
                if o2[j][i] == '1' {
                    if j as isize > o2.len() as isize - j as isize {
                        o2.truncate(j);
                        break;
                    }
                    else {
                        o2 = o2 [j..].to_vec();
                        break;
                    }
                }
            }
        }
        i32::from_str_radix(&o2[0].iter().collect::<String>(), 2).unwrap()
    });

    let co2_thread = thread::spawn(move || {
        for i in 0..co2[0].len() {
            if co2.len() == 1 { break }

            co2.sort_by(|a,b| a[i].cmp(&b[i]));

            for j in 0..co2.len() {
                if co2[j][i] == '1' {
                    if j as isize > co2.len() as isize - j as isize {
                        co2 = co2 [j..].to_vec();
                        break;
                    }
                    else {
                        co2.truncate(j);
                        break;
                    }
                }
            }
        }
        i32::from_str_radix(&co2[0].iter().collect::<String>(), 2).unwrap()
    });

    let o2_value = o2_thread.join().unwrap();
    let co2_value = co2_thread.join().unwrap();

    println!("Part 2: {}", o2_value * co2_value);
}
