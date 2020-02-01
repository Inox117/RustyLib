pub mod handlers;
pub mod repository;
use super::schema::books;
use diesel::sql_types::Date;

#[derive(Debug, PartialEq, Eq)]
enum Classification {
    Manga,
    Comics,
    Book,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "books"]
pub struct Book {
    pub id: i32,
    pub isbn10: String,
    pub isbn13: String,
    pub title: String,
    pub author: String,
    pub summary: String,
    pub illustrator: String,
    pub publication_date: Date,
    pub publisher: String,
    pub number_of_page: i32,
    pub classification: Classification,
}
