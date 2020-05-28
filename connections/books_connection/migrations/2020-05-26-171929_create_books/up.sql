-- Your SQL goes here

CREATE TABLE books(
    id SERIAL PRIMARY KEY,
    book_uid VARCHAR NOT NULL, 
    title VARCHAR NOT NULL,
    author VARCHAR NOT NULL,
    year SMALLINT NOT NULL
)