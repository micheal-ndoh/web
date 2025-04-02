CREATE TABLE compression_tasks (
    id SERIAL PRIMARY KEY,
    file_name TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending'
);

CREATE INDEX idx_compression_tasks_status ON compression_tasks(status);