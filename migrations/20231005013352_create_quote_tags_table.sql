-- Add migration script here
CREATE TABLE   IF NOT EXISTS quote_tags (
    id SERIAL NOT NULL,
    tag varchar(100) NOT NULL UNIQUE
);