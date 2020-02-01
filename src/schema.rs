table! {
    books (id) {
        id -> Integer,
        isbn10 -> Varchar,
        isbn13 -> Varchar,
        title -> Varchar,
        author -> Varchar,
        illustrator -> Varchar,
        summary -> Mediumtext,
        publisher -> Varchar,
        publication_date -> Date,
        number_of_page -> Integer,
        classification -> Enum,
    }
}
