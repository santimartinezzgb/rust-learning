// use std::io;

#[derive(Debug)]
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

#[derive(Debug)] // This attribute makes the struct printable using {:?}
struct Library {
    name: String,
    books: Vec<Book>,
}
impl Library {
    fn new(name: String) -> Self {
        Library {
            name,
            books: Vec::new(),
        }
    }
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }
}

fn main() {
    let mut library = Library::new("Code Library".to_string());

    let book_1 = Book::new(
        "Clean code".to_string(),
        "Robert C. Martin".to_string(),
        true,
    );
    let book_2 = Book::new(
        "The Architect".to_string(),
        "Robert C. Martin".to_string(),
        false,
    );

    library.add_book(book_1); // In this moment the book is moved to library, so we can't use it anymore here. This is ownership in action.
                              // On rust. Any value has a single owner.
    library.add_book(book_2);

    println!("Library's name: {}", library.name);
    for book in &library.books {
        // If you not use & here, the book will be moved from library to this loop, and you can't use library anymore.
        println!(
            "Book: {}, Author: {}, Available: {}",
            book.title, book.author, book.available
        );
    }
}
