-- Add migration script here
CREATE TABLE quote_tags (
    id SERIAL NOT NULL,
    tag_id int NOT NULL,
    quote_id int NOT NULL
);