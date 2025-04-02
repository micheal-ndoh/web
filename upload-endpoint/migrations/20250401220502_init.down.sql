-- Add up migration script here

-- Completely undo the migration
DROP INDEX IF EXISTS idx_compression_tasks_status;
DROP TABLE IF EXISTS compression_tasks;