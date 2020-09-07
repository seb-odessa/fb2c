
CREATE TABLE archives (
  id INTEGER NOT NULL PRIMARY KEY,
  zip_name TEXT NOT NULL,
  zip_path TEXT NOT NULL,
  zip_md5  TEXT NOT NULL,
  CONSTRAINT u_zip_md5 UNIQUE(zip_md5) ON CONFLICT IGNORE
);

CREATE TABLE authors (
  id INTEGER NOT NULL PRIMARY KEY,
  first_name TEXT NOT NULL,
  middle_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  nickname TEXT NOT NULL,
  uuid TEXT NOT NULL,
  CONSTRAINT u_authors UNIQUE(first_name, middle_name, last_name, nickname, uuid) ON CONFLICT IGNORE
);

CREATE TABLE books (
  id INTEGER NOT NULL PRIMARY KEY,
  book_title TEXT NOT NULL
);

