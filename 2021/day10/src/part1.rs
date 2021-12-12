use std::collections::HashMap;

use regex::Regex;

pub fn part1(input: &Vec<&str>, legal_lines: &mut Vec<String>) {
    let mut sum = 0;

    'outer: for line in input.clone() {
        // let mut counter = HashMap::new();
        // counter.insert('(', 0);
        // counter.insert('[', 0);
        // counter.insert('{', 0);
        // counter.insert('<', 0);

        // for bracket in line.chars() {
        //     match counter.get_mut(&bracket) {
        //         Some(entry) => *entry += 1,
        //         None => {
        //             let (entry, _) = match bracket {
        //                 ')' => (counter.get_mut(&'(').unwrap(), 3),
        //                 ']' => (counter.get_mut(&'[').unwrap(), 57),
        //                 '}' => (counter.get_mut(&'{').unwrap(), 1197),
        //                 '>' => (counter.get_mut(&'<').unwrap(), 25137),
        //                 _ => panic!("Invalid character"),
        //             };

        //             *entry -= 1;
        //         }
        //     }
        // }

        let mut text = line.to_string();

        let re = Regex::new(r"(\(\)|\[\]|\{\}|<>)").unwrap();

        let mut has_changed = true;

        while has_changed {
            has_changed = false;
            let new_text = re.replace_all(&text, "").to_string();
            if new_text != text {
                text = new_text;
                has_changed = true;
            }
        }
 
        for bracket in text.chars() {
            match bracket {
                ')' => {sum += 3; continue 'outer},
                ']' => {sum += 57; continue 'outer},
                '}' => {sum += 1197; continue 'outer},
                '>' => {sum += 25137; continue 'outer},
                _ => (),
            }
        }

        legal_lines.push(text);
    }
    println!("Part 1: {}", sum);
}
