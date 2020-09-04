-- Your SQL goes here
CREATE TABLE authors (
  id INTEGER NOT NULL PRIMARY KEY,
  first_name TEXT NOT NULL,
  middle_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  nickname TEXT NOT NULL,
  uuid TEXT NOT NULL,
  CONSTRAINT unique_authors UNIQUE(first_name, middle_name, last_name, nickname, uuid) ON CONFLICT IGNORE
);

CREATE TABLE books (
  id INTEGER NOT NULL PRIMARY KEY,
  book_title TEXT NOT NULL
);

