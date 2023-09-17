use super::task_1::*;

impl<'a> Book<'a> {
    pub fn new(title: &str, author: &str) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            is_available: true,
            borrowed_by: None,
        }
    }
}

impl<'a> Library<'a> {
    pub fn borrow_book(&mut self, borrower: &'a Person, book_title: String) {
        println!("{} is trying to borrow {}", borrower.name, book_title);
        let is_book_found = self.books.iter_mut().find(|book| book.title == book_title);
        match is_book_found {
            Some(book) => {
                if book.is_available {
                    book.borrowed_by = Some(borrower);
                    book.is_available = false;
                    println!("Success: {} borrowed by {}", book.title, borrower.name)
                } else {
                    println!("Error: Book - {} not available to borrow", book.title);
                }
            }
            None => println!("Error: Book - {} not found", book_title),
        }
        println!("");
    }

    pub fn return_book(&mut self, book_title: String) {
        println!("{} is being returned", book_title);
        let is_book_found = self.books.iter_mut().find(|book| book.title == book_title);
        match is_book_found {
            Some(book) => {
                if !book.is_available {
                    book.borrowed_by = None;
                    book.is_available = true;
                    println!("Book - {} returned to library", book.title);
                } else {
                    println!("Error: Book - {} was never borrowed", book.title);
                }
            }
            None => println!("Error: Book - {} not part of library", book_title),
        }
        println!("");
    }

    pub fn list_all_available_books(&self) {
        let available_books: Vec<_> = self.books.iter().filter(|book| book.is_available).collect();

        println!("\nAVAILABLE BOOKS");
        for book in available_books {
            println!("{}", book.title)
        }
        println!("")
    }

    pub fn list_all_borrowed_books(&self) {
        let borrowed_books: Vec<_> = self
            .books
            .iter()
            .filter(|book| !book.is_available)
            .collect();

        println!("\nBORROWED BOOKS");
        for book in borrowed_books {
            println!("{}", book.title)
        }
        println!("")
    }
}
