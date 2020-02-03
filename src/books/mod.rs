pub mod handlers;
pub mod repository;
use super::schema::books;
use diesel::deserialize::Queryable;

type DB = diesel::pg::Pg;

#[derive(AsChangeset, Identifiable, Debug, PartialEq, Serialize, Deserialize)]
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

impl Queryable<books::SqlType, DB> for Book {
    type Row = (i32, String, String, String, String, String, String, String, String, i32);

    fn build(row: Self::Row) -> Self {
        Book {
            id: row.0,
            isbn10: row.1,
            isbn13: row.2,
            title: row.3,
            author: row.4,
            illustrator: row.5,
            summary: row.6,
            publisher: row.7,
            publication_date: row.8,
            number_of_page: row.9
        }
    }
}

