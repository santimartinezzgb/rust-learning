// OWNERSHIP. Basic example of ownership transfer in Rust.
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

fn main() {
    {
        let s1 = String::from("Hello World");
        let _s2 = s1; // s1 is moved to s2
                      // _ indicates that s2 is intentionally unused

        // println!("s1 = {}", s1); This line would cause a compile-time error because s1 is no longer valid
    }
    {
        let s1 = String::from("Hello World");
        let s2 = &s1;

        // When using references (&), ownership is not transferred, the pointer is borrowed
        //println!("s1 = {}, s2 = {}", s1, s2);
    }
    {
        let s1 = String::from("Hello World");
        let s2 = s1.clone(); // s1 is cloned to s2

        //println!("s1 = {}, s2 = {}", s1, s2); // Both s1 and s2 are valid
    }
    {
        let s = String::from("Hello World");
        takes_ownership(s);

        let x = 5;
        makes_copy(x);
    }
}

fn takes_ownership(some_string: String ){
    println!("{some_string}");
}
fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
