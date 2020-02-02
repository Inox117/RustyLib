-- Your SQL goes here
CREATE TABLE books(
  id SERIAL PRIMARY KEY,
  isbn10 VARCHAR(10) UNIQUE NOT NULL,
  isbn13 VARCHAR(13) UNIQUE NOT NULL,
  title VARCHAR(255) NOT NULL,
  author VARCHAR(255) NOT NULL,
  illustrator VARCHAR(255) NOT NULL,
  summary TEXT NOT NULL,
  publisher VARCHAR(255) NOT NULL,
  publication_date VARCHAR(255) NOT NULL,
  number_of_page INTEGER NOT NULL
)
