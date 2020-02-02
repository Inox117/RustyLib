table! {
    books (id) {
        id -> Int4,
        isbn10 -> Varchar,
        isbn13 -> Varchar,
        title -> Varchar,
        author -> Varchar,
        illustrator -> Varchar,
        summary -> Text,
        publisher -> Varchar,
        publication_date -> Varchar,
        number_of_page -> Int4,
    }
}
