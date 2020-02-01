-- Your SQL goes here
CREATE TABLE books(
  id INTEGER AUTO_INCREMENT PRIMARY KEY,
  isbn10 VARCHAR(14) UNIQUE NOT NULL,
  isbn13 VARCHAR(18) UNIQUE NOT NULL,
  title VARCHAR(255) NOT NULL,
  author VARCHAR(255) NOT NULL,
  illustrator VARCHAR(255) NOT NULL,
  summary TEXT(65535) NOT NULL,
  publisher VARCHAR(255) NOT NULL,
  publication_date DATE NOT NULL,
  number_of_page INTEGER NOT NULL,
  classification ENUM ('Manga', 'Comics', 'Book') NOT NULL
)
