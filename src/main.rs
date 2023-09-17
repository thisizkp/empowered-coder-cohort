mod assignments;

use assignments::task_1::*;

fn main() {
    let mut library = Library {
        books: vec![
            Book::new("Programming Rust", "Jim Blandy"),
            Book::new("Designing Data Intensive Applications", "Martin Kleppmann"),
            Book::new("Introduction to Algorithms", "CLRS"),
        ],
    };

    library.list_all_available_books();

    let first_borrower = Person {
        name: "KP".to_string(),
        age: 30,
    };
    library.borrow_book(&first_borrower, "Concrete Mathematics".to_string());
    library.borrow_book(&first_borrower, "Programming Rust".to_string());

    let second_borrower = Person {
        name: "Ghost KP".to_string(),
        age: 99,
    };
    library.borrow_book(&second_borrower, "Programming Rust".to_string());
    library.borrow_book(&second_borrower, "Introduction to Algorithms".to_string());

    library.return_book("Introduction to Algorithms".to_string());

    library.list_all_available_books();
    library.list_all_borrowed_books();
}
