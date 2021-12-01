use std::{sync::{Arc, Condvar, Mutex, mpsc::channel}, thread, time::Duration};

use md5::compute;
use part1::part1;
use part2::part2;
use utils::input::read_input;

mod part1;
mod part2;
mod structs;

fn main() {
    let input = read_input("assets/input.txt").unwrap();

    let (sender, receiver) = channel();
    let (sender2, receiver2) = channel();

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    let hasher = thread::spawn(move || {
        let (lock, cvar) = &*pair;
        let mut finished = lock.lock().unwrap();

        for i in 0.. {
            let digest = compute(format!("{}{}", &input, i));
            let hex = format!("{:x}", digest);

            if &hex[..5] == "00000" {
                println!("{}{} -> {}", input, i, hex);
                match sender.send(hex.clone()) {
                    _ => ()
                };
                sender2.send(hex).unwrap();
            }

            let result = cvar.wait_timeout(finished, Duration::from_millis(0)).unwrap();
            finished = result.0;
            if *finished {
                return;
            }
        }
    });

    let part1_thread = thread::spawn(move || {
        part1(&receiver)
    });

    let part2_thread = thread::spawn(move || {
        part2(&receiver2, pair2)
    });

    hasher.join().unwrap();
    println!("Part 1: {}", part1_thread.join().unwrap());
    println!("Part 2: {}", part2_thread.join().unwrap());
}
