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
use rocket::routes;
use rocket_contrib::templates::Template;

mod books;
mod schema;
mod db_connection;

fn main() {
    dotenv().ok();
    create_routes();
}

pub fn create_routes() {
    rocket::ignite()
        .attach(Template::fairing())
        .manage(db_connection::init_pool())
        .mount("/books",
               routes![books::handlers::get_all,
                              books::handlers::get,
                              books::handlers::post,
                              books::handlers::put,
                              books::handlers::delete,
                              books::handlers::index],
        ).launch();
}