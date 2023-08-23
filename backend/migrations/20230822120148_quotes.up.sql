-- Add up migration script here
-- Create a new table to store daily quotes
CREATE TABLE IF NOT EXISTS quotes (
    ticker VARCHAR(8) NOT NULL,
    date DATE NOT NULL,
    open NUMERIC(10,2) NOT NULL,
    high NUMERIC(10,2) NOT NULL,
    low NUMERIC(10,2) NOT NULL,
    close NUMERIC(10,2) NOT NULL,
    volume BIGINT NOT NULL,
    PRIMARY KEY (ticker, date)
);

-- Create an index on the ticker column for faster lookups
CREATE INDEX quote_ticker ON quotes (ticker);