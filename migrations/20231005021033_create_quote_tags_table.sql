-- Add migration script here
CREATE TABLE quote_tags (
    id SERIAL PRIMARY KEY NOT NULL,
    tag_id int NOT NULL,
    quote_id int NOT NULL,
    FOREIGN KEY(tag_id) REFERENCES tags(id) ON DELETE CASCADE,
    FOREIGN KEY(quote_id) REFERENCES quotes(id) ON DELETE CASCADE
);