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
for file in tqdm(sorted(Path(directory).glob("*.csv")), desc="Processing files"):
    try:
        df = pl.read_csv(file)
        df = df.filter((pl.col("Open") > 0) & (pl.col("Close") > 0))
    except Exception as e:
        print(f"Error processing {file.stem}: {e}")
        continue
# build a dictionary of returns, keyed by time period
# and bucketed by return value, counting the number of instances
    df = df.sort("Date")
    current_date_y = datetime.strptime(df["Date"].min(), "%Y-%m-%d")
    current_date_m = datetime.strptime(df["Date"].min(), "%Y-%m-%d")
    current_date_w = datetime.strptime(df["Date"].min(), "%Y-%m-%d")
    open_price_y = df["Open"][0]
    open_price_m = df["Open"][0]
    open_price_w = df["Open"][0]
    for row in df.rows():
        date = datetime.strptime(row[0], "%Y-%m-%d")
        if (row[1] == 0):
            continue
        return_value = round(row[4] / row[1], 3)
        if return_value not in returns["daily"]:
            returns["daily"][return_value] = 0
        returns["daily"][return_value] += 1
        
        if date - current_date_w > timedelta(days=7):
            current_date_w = date
            return_value = round(row[4] / open_price_w, 3)
            open_price_w = row[1]
            if return_value not in returns["weekly"]:
                returns["weekly"][return_value] = 0
            returns["weekly"][return_value] += 1
            
        if date - current_date_m > timedelta(days=30):
            current_date_m = date
            return_value = round(row[4] / open_price_m, 3)
            open_price_m = row[1]
            if return_value not in returns["monthly"]:
                returns["monthly"][return_value] = 0
            returns["monthly"][return_value] += 1
            
        if date - current_date_y > timedelta(days=365):
            current_date_y = date
            return_value = round(row[4] / open_price_y, 3)
            open_price_y = row[1]
            if return_value not in returns["yearly"]:
                returns["yearly"][return_value] = 0
            returns["yearly"][return_value] += 1
    
json.dump(returns, open("data/simple_returns.json", "w"))