-- Add up migration script here
CREATE TABLE todo_items (
    id INTEGER PRIMARY KEY,
    checked BOOLEAN NOT NULL,
    contents TEXT NOT NULL
);