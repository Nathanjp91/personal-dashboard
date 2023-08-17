import polars as pl
from datetime import datetime, timedelta
from pathlib import Path
import json
from tqdm import tqdm
directory = "data/stocks"

returns = {
    "yearly": {},
    "monthly": {},
    "weekly": {},
    "daily": {},
}

all_dates = set()
columns = set()
for file in tqdm(sorted(Path(directory).glob("*.csv")), desc="Processing Dates"):
    df = pl.read_csv(file)
    try:
        df = pl.read_csv(file)
        df = df.filter((pl.col("Open") > 0) & (pl.col("Close") > 0))
    except Exception as e:
        print(f"Error processing {file.stem}: {e}")
        continue
    columns.add(file.stem)
    all_dates.update(df["Date"])
all_dates = sorted(all_dates)
columns = sorted(columns)
data = {
    "Date": all_dates,
    # **{column: [None] * len(all_dates) for column in columns}
}
all_df = pl.DataFrame(
    data
)
for file in tqdm(sorted(Path(directory).glob("*.csv")), desc="Processing Returns"):
    df = pl.read_csv(file)
    try:
        df = pl.read_csv(file)
        df = df.filter((pl.col("Open") > 0) & (pl.col("Close") > 0))
    except Exception as e:
        print(f"Error processing {file.stem}: {e}")
        continue
    df = df.with_columns((pl.col("Close") / pl.col("Open")).alias(file.stem))
    df = df.select(["Date", file.stem])
    all_df = all_df.join(df, on="Date", how="outer")

ticker_counts = all_df.apply(lambda row: sum(1 for value in row if value))
all_df = all_df.with_columns(ticker_counts.rename({'apply': 'ticker_count'}))
all_df = all_df.with_columns(pl.all().fill_null(strategy="zero"))
all_df = all_df.with_columns(pl.sum_horizontal(*columns).alias("sum"))
all_df = all_df.with_columns((pl.col("sum") / pl.col('ticker_count')).alias("avg"))
all_df.to_csv("data/stocks/aligned.csv")
    