#[allow(dead_code)]
pub struct Person {
    name: String,
    age: u8,
}

#[allow(dead_code)]
pub struct Book {
    title: String,
    author: String,
    is_available: bool,
}

#[allow(dead_code)]
pub struct Library {
    books: Vec<Book>,
}
