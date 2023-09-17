#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

#[derive(Debug)]
pub struct Book<'a> {
    pub title: String,
    pub author: String,
    pub is_available: bool,
    pub borrowed_by: Option<&'a Person>,
}

pub struct Library<'a> {
    pub books: Vec<Book<'a>>,
}
