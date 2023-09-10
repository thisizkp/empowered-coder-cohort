struct Person {
    name: String,
    age: u8
}

struct Book {
    title: String,
    author: String,
    is_available: bool
}

struct Library {
    books: Vec<Book>
}