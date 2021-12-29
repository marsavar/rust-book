use std::{thread, sync::mpsc};
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![String::from("Hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread")];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in rx {
    println!("Received \"{}\"", received);
    }
}
