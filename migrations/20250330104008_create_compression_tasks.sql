-- Add migration script here

CREATE TABLE compression_tasks (
    id SERIAL PRIMARY KEY,
    file_name TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('pending', 'completed', 'failed'))
);
