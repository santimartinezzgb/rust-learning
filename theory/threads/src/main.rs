use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Exercise 1: Basic Thread ===");
    basic_thread();

    println!("=== Exercise 2: Join Handle ===");
    //join_handle();

    println!("=== Exercise 3: Move Closure ===");
    //move_closure();

    println!("=== Exercise 4: Create multiple threads ===");
    //create_multiple_threads();

    println!("=== Exercise 5: Share data between threads ===");
    //share_data_threads();
}

fn basic_thread() {
    thread::spawn(|| {
        println!("Hello from secondary thread");
    }); // Spawn a new thread || This works like a closure

    thread::sleep(Duration::from_millis(2000)); // Without this the primary thread ends before the secondary, so we don't see the output
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
            i + 1 // Without ; - this is a return expresion
        });

        // Push threads to vector
        handles.push(handle);
    }

    // Print the thread results
    for handle in handles {
        let result = handle.join().unwrap();
        println!("Result: {}", result);
    }
}

// 5. Share data between threads with Arc<Mutex<T>>
fn share_data_threads() {
    // Create a secure shared counter between threads
    // Arc: share
    // Mutex: protect
    // Arc = Shared link you give to 10 people
    // Mutex = The lock that only allows one person to edit it at a time
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // &counter. Clone a counter reference, not his value
        // VERY IMPORTANT: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html?highlight=OWNERSHIP#what-is-ownership
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // .lock() to use a value Mutex
            let mut num = counter_clone.lock().unwrap();
            // *. To access inside mutex
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter result: {}", *counter.lock().unwrap());
}
