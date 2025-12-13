use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Exercise 1: Basic Thread ===");
    basic_thread();

    println!("=== Exercise 2: Join Handle ===");
    join_handle();
}

fn basic_thread() {
    thread::spawn(|| {
        println!("Hello from secondary thread");
    });

    thread::sleep(Duration::from_millis(2000));
    println!("Hello from primary thread");
}

fn join_handle() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Number {} from secondary thread", i);
            thread::sleep(Duration::from_millis(2000));
        }
    });

    for i in 1..5 {
        println!("Number {} from primary thread", i);
        thread::sleep(Duration::from_millis(2000));
    }

    handle.join().unwrap();
    println!("Secondary thread finished");
}
