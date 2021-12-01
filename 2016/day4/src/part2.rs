use crate::structs::Room;

pub fn part2(input: &Vec<Room>) {
    // let mut vec = Vec::new();
    
    for room in input {
        let letters = room.name.chars();
        let mut out_string = String::new();

        for mut letter in letters {
            let mut utf8 = [0u8];
            letter.encode_utf8(&mut utf8);
            
            let new_utf8 = ((utf8[0] as i32 - 97 + room.id) % 26 + 97) as u32;
            letter = char::from_u32(new_utf8).unwrap();

            out_string.push(letter);
        }
        if out_string == "northpoleobjectstorage".to_string() {
            println!("Part 2: {}", room.id);
            return;
        }

        // vec.push(out_string);
    }
    println!("Part 2: error");

    // vec.sort();
    // for room in vec {
    //     println!("{}", room);
    // }
}