-- Add migration script here
CREATE TABLE IF NOT EXISTS quotes (
    id SERIAL NOT NULL,
    quote text NOT NULL,
    author_id int NOT NULL,
    created_by int NOT NULL,
    likes_count int NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);