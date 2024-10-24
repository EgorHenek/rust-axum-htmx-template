-- Add up migration script here
CREATE TABLE variants (
    id BLOB PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    confirmed BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
