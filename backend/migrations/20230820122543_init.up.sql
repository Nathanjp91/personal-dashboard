-- Add up migration script here
-- Create a new table to store financial data
CREATE TABLE IF NOT EXISTS stocks (
    id SERIAL PRIMARY KEY,
    ticker VARCHAR(8) NOT NULL,
    amount_held INT NOT NULL,
    last_updated DATE NOT NULL
);

-- Create an index on the ticker column for faster lookups
CREATE INDEX stocks_ticker ON stocks (ticker);