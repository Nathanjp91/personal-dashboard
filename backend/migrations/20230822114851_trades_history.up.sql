-- Add up migration script here
-- Create a new table to store stock trade history
CREATE TABLE IF NOT EXISTS trades_history (
    id SERIAL PRIMARY KEY,
    ticker VARCHAR(8) NOT NULL,
    amount INT NOT NULL,
    trade_type VARCHAR(4) NOT NULL,
    date DATE NOT NULL,
    country VARCHAR(2) NOT NULL,
    price NUMERIC(10,2) NOT NULL
);

-- Create an index on the ticker column for faster lookups
CREATE INDEX trades_ticker ON trades_history (ticker);