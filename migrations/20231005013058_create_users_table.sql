-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY NOT NULL,
    username varchar(255) NOT NULL,
    password varchar(255) NOT NULL,
    token varchar(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);