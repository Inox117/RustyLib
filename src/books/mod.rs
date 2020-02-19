pub mod handlers;
pub mod repository;
use super::schema::books;
use diesel::deserialize::Queryable;
use std::str::FromStr;
use std::fmt::Display;
use serde::de::{self, Deserializer, Deserialize};

type DB = diesel::pg::Pg;

#[derive(AsChangeset, Identifiable, Debug, PartialEq, Serialize, Deserialize)]
#[table_name = "books"]
pub struct Book {
    #[serde(skip_deserializing)] // When receiving data form a POST id is not available.
    // To avoid maintaining 2 struct I prefer a default value : 0 From std::default::Default
    id: i32,
    isbn10: String,
    isbn13: String,
    title: String,
    author: String,
    summary: String,
    illustrator: String,
    publication_date: String,
    publisher: String,
    #[serde(deserialize_with = "from_str")] //No implicit conversion
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

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

#[derive(Insertable)]
#[table_name = "books"]
pub struct InsertableBook {
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

impl InsertableBook {

    fn from_book(book: Book) -> InsertableBook {
        InsertableBook {
            isbn10: book.isbn10,
            isbn13: book.isbn13,
            title: book.title,
            author: book.author,
            summary: book.summary,
            illustrator: book.illustrator,
            publication_date: book.publication_date,
            publisher: book.publisher,
            number_of_page: book.number_of_page
        }
    }
}