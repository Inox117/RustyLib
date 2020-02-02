pub mod handlers;
pub mod repository;
use super::schema::books;

#[derive(Queryable, AsChangeset, Identifiable, Debug, PartialEq, Serialize, Deserialize)]
#[table_name = "books"]
pub struct Book {
    id: i32,
    isbn10: String,
    isbn13: String,
    title: String,
    author: String,
    summary: String,
    illustrator: String,
    publication_date: String,
    publisher: String,
    number_of_page: i32,
}
