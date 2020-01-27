#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;

mod books;
mod schema;
mod db_connection;


fn main() {
    dotenv().ok();
    books::handlers::create_routes();
}