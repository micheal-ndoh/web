
CREATE TABLE IF NOT EXISTS compression_tasks (
    id SERIAL PRIMARY KEY,
    file_name TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending'
);

CREATE INDEX compression_tasks_file_name_idx ON compression_tasks (file_name);