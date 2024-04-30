use std::fs::File;
use std::io::{stdin, ErrorKind};
use std::time::Duration;
use std::{thread, u32};
fn main() {
    let user_profile = File::open("user_data.txt");
    let mut tracker = 0;
    thread::spawn(move || {
        for x in 0..10 {
            tracker += 1;
            thread::sleep(Duration::from_millis(1));
        }
    });
    println!("current Thread {}", tracker);
    thread_host(tracker);
}
fn thread_host(check: u32) {
    println!("The current Thread {}", check);
}

fn do_task() {}
