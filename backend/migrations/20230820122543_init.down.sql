-- Add down migration script here
-- Drop the index
DROP INDEX IF EXISTS idx_ticker;

-- Drop the table
DROP TABLE IF EXISTS stocks;