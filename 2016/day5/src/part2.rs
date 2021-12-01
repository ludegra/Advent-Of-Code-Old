use std::sync::{Arc, Condvar, Mutex, mpsc::Receiver};

pub fn part2(reciever: &Receiver<String>, terminator: Arc<(Mutex<bool>, Condvar)>) -> String {
    let (lock, cvar) = &*terminator;

    let mut output = vec!['\0'; 8];
    let mut counter = 0;

    for recieved in reciever {
        let mut chars = recieved.chars();

        let index = chars.nth(5).unwrap();
        let value = chars.next().unwrap();

        if index.is_digit(10) {
            let index = index.to_digit(10).unwrap() as usize;

            if index < output.len() {
                if output[index] == '\0' {
                    output[index] = value;
                    counter += 1;
                }
            }
        }
        if counter == 8 {
            break;
        }
    }

    let mut finished = lock.lock().unwrap();
    *finished = true; 

    cvar.notify_one();

    output.into_iter().collect::<String>()
}