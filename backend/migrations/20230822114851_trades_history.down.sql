-- Add down migration script here
-- Drop the index
DROP INDEX IF EXISTS trades_ticker;

-- Drop the table
DROP TABLE IF EXISTS trades_history;