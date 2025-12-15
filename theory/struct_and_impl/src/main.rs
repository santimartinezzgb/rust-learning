// use std::io;

struct Book {
    title: String,
    author: String,
    available: bool,
}

impl Book {
    fn new(title: String, author: String, available: bool) -> Self {
        Book {
            title,
            author,
            available,
        }
    }
}

fn main() {
    let book = Book::new(
        "Clean code".to_string(),
        "Robert C. Martin".to_string(),
        true,
    );
    println!("Book's title: {}\n", book.title);
    println!("Author's title: {}\n", book.author);
    println!("Is available?: {}\n", book.available);
}
