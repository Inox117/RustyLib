pub mod handlers;
pub mod repository;
use super::schema::books;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "books"]
pub struct Book {
    pub id: i32,
    pub isbn: String,
    pub title: String,
    pub author: String,
}
