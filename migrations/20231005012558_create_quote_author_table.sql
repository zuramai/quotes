-- Add migration script here
CREATE TABLE IF NOT EXISTS quote_authors (
    id SERIAL NOT NULL,
    name varchar(255) NOT NULL,
    slug varchar(255) NOT NULL
);