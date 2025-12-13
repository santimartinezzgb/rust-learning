use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Exercise 1: Basic Thread ===");
    basic_thread();

    println!("=== Exercise 2: Join Handle ===");
    join_handle();

    println!("=== Exercise 3: Move Closure ===");
    move_closure();

    println!("=== Exercise 4: Create multiple threads ===");
    create_multiple_threads();
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

// 3. Move data to thread
fn move_closure() {
    // Array: when you know before the size
    // Vec: when you need a collection that can grow or shrink
    let v = vec![1, 2, 3, 4, 5];

    let handle = thread::spawn(move || {
        for i in v {
            println!("Vector from thread {}", i);
            thread::sleep(Duration::from_millis(2000));
        }
    });

    handle.join().unwrap();
}

// 4. Create multiple threads
fn create_multiple_threads() {
    let mut handles = vec![];

    for i in 0..5 {
        // Create 5 threads
        let handle = thread::spawn(move || {
            println!("Thread {} working...", i);
            thread::sleep(Duration::from_millis(2000));
            i * 2
        });

        // Push threads to vector
        handles.push(handle);
    }

    // Print the thread results
    for handle in handles {
        let resultado = handle.join().unwrap();
        println!("Result: {}", resultado);
    }
}
