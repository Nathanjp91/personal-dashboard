-- Add down migration script here
-- Drop the index
DROP INDEX IF EXISTS quote_ticker;

-- Drop the table
DROP TABLE IF EXISTS quotes;