-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY NOT NULL,
    username varchar(255),
    password varchar(255),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);