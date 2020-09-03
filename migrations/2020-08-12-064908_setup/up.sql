-- Your SQL goes here
CREATE TABLE authors (
  id INTEGER NOT NULL PRIMARY KEY,
  first_name TEXT,
  middle_name TEXT,
  last_name TEXT,
  nickname TEXT,
  lib_id TEXT,
  CONSTRAINT unique_authors UNIQUE(first_name, middle_name, last_name, nickname) ON CONFLICT IGNORE
);

CREATE TABLE homepages (
    id INTEGER NOT NULL PRIMARY KEY,
    owner id INTEGER NOT NULL,
    homepage TEXT NOT NULL,
    FOREIGN KEY(owner) REFERENCES authors(id),
    CONSTRAINT unique_homepages UNIQUE(homepage) ON CONFLICT IGNORE
);

CREATE TABLE emails (
    id INTEGER NOT NULL PRIMARY KEY,
    owner id INTEGER NOT NULL,
    email TEXT NOT NULL,
    FOREIGN KEY(owner) REFERENCES authors(id),
    CONSTRAINT unique_emails UNIQUE(email) ON CONFLICT IGNORE
);

