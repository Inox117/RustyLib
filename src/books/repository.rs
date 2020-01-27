#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use crate::schema::books;
use crate::books::Book;

pub fn all(connection: &MysqlConnection) -> QueryResult<Vec<Book>> {
    books::table.load::<Book>(&*connection)
}

pub fn get(id: i32, connection: &MysqlConnection) -> QueryResult<Book> {
    books::table.find(id).get_result::<Book>(connection)
}

pub fn insert(book: Book, connection: &MysqlConnection) -> QueryResult<Book> {
    use crate::schema::books::dsl::*;
    diesel::insert_into(books)
        .values(&InsertableBook::from_book(book))
        .execute(connection);
    books.order(id).first(connection)
}

pub fn update(id: i32, book: Book, connection: &MysqlConnection) -> QueryResult<Book> {
    diesel::update(books::table.find(id))
        .set(&book)
        .execute(connection);
    books::table.find(id).get_result::<Book>(connection)
}

pub fn delete(id: i32, connection: &MysqlConnection) -> QueryResult<usize> {
    diesel::delete(books::table.find(id))
        .execute(connection)
}

#[derive(Insertable)]
#[table_name = "books"]
struct InsertableBook {
    title: String,
    author: String,
}

impl InsertableBook {

    fn from_book(book: Book) -> InsertableBook {
        InsertableBook {
            title: book.title,
            author: book.author,
        }
    }
}