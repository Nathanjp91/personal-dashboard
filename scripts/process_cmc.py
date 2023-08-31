import polars as pl
import requests
import re
from datetime import datetime
df = pl.read_csv("./data/cmc_trading_account.csv")
server = "http://localhost:8080/trades"

totals = {}
for row in df.iter_rows():
    raw_type = row[2]
    if raw_type != "CB" and raw_type != "CS":
        continue
    trade_type = "Buy" if raw_type == "CB" else "Sell"
    date = datetime.strptime(row[0], "%d/%m/%Y").strftime("%Y-%m-%d")
    results = re.match(r"(Sold|Bght) (\d+) ([\w:]*) @ ([\d\.]*)", row[3])
    ticker = results.group(3).strip() + ".AX" if not results.group(3).endswith(":US") else results.group(3)[:-3]
    amount = int(results.group(2))
    price = float(results.group(4))
    body = {
        "ticker": ticker,
        "date": date,
        "trade_type": trade_type,
        "amount": amount,
        "country": "AU",
        "price": price
    }
    if body['ticker'] not in totals:
        totals[body['ticker']] = 0
    if body['trade_type'] == 'Buy':
        totals[body['ticker']] += body['amount']
    elif body['trade_type'] == 'Sell':
        totals[body['ticker']] -= body['amount']
    else:
        raise ValueError("Trade type doesn't exist")
    
    response = requests.post(server, json=body)
    if response.status_code != 200:
        print(response.text + "\n" + str(body))

print(totals)
        