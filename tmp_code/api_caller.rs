use crate::books::Book;
use std::fmt;
use http::{Request, Response};
use hyper::Client;
use std::collections::HashMap;

pub fn get_book_from_open_library(isbn13: String) -> Book {
    let sanitize_isbn = isbn13.chars()
        .map(|x| match x {
            '-' => "",
            _ => x
        }).collect();
    let url_to_call = format!("https://openlibrary.org/api/books?bibkeys=ISBN:{}&jscmd=details&format=json", sanitize_isbn);
}

#[tokio::main]
pub async fn call(url_to_call: String) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(url_to_call)
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}