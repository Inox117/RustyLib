use std::env;

use diesel::result::Error;
use rocket::{delete, get, post, put};
use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket::response::status;

use crate::books;
use crate::books::Book;
use crate::db_connection::DbConn;

use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/")]
pub fn get_all(connection: DbConn) -> Template {
    let book_list = books::repository::all(&connection)
        .map(|book| book)
        .map_err(|error| println!("{:?}", error));
//    error_status(error)
    let mut context =  HashMap::new();
    context.insert("books", book_list.unwrap());
    Template::render("books", context)
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Template {
    let book = books::repository::get(id, &connection)
        .map_err(|error| error_status(error));
    let mut context = HashMap::new();
    context.insert("book", book.unwrap());
    Template::render("book", context)
}

#[post("/", format = "application/json", data = "<book>")]
pub fn post(book: Json<Book>, connection: DbConn) -> Result<status::Created<Json<Book>>, Status> {
    books::repository::insert(book.into_inner(), &connection)
        .map(|book| book_created(book))
        .map_err(|error| error_status(error))
}

#[put("/<id>", format = "application/json", data = "<book>")]
pub fn put(id: i32, book: Json<Book>, connection: DbConn) -> Result<Json<Book>, Status> {
    books::repository::update(id, book.into_inner(), &connection)
        .map(|book| Json(book))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match books::repository::get(id, &connection) {
        Ok(_) => books::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}

fn book_created(book: Book) -> status::Created<Json<Book>> {
    status::Created(
        format!("{host}:{port}/book/{id}", host = host(), port = port(), id = book.isbn13).to_string(),
        Some(Json(book)))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}