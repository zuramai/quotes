-- Add migration script here
CREATE TABLE   IF NOT EXISTS tags (
    id SERIAL PRIMARY KEY NOT NULL,
    tag varchar(100) NOT NULL UNIQUE
);