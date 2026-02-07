"""
Calculates price return for an ETF over a period of time
Splits and ticker changes are considered
Takes ETF ticker and timeframe as inputs
"""
import sys
import csv
from datetime import date, timedelta, datetime
from collections import defaultdict

TIMEFRAMES = {
    "1 day": 1,
    "5 days": 5,
    "6 months": 182,
    "1 year": 365
}

price_data = None
splits_data = None
ticker_changes_data = None
portfolio_data = None


def calc_price_return(end_price, start_price):
    return (end_price / start_price - 1) * 100


def handle_ticker_change(ticker):
    changed_tickers = []
    for _, changed_ticker in ticker_changes_data[ticker]:
        price_data[ticker].update(price_data[changed_ticker])
        changed_tickers.append(changed_ticker)
    return changed_tickers


def handle_split_calculation(ticker, start_date, end_date, price):
    splits = splits_data[ticker]
    adjusted_price = price
    for split_date_str, split_ratio in splits.items():
        split_date = datetime.strptime(split_date_str, "%d/%m/%Y").date()
        # Only action splits that have occurred within timeframe
        if start_date < split_date < end_date:
            from_quantity, to_quantity = split_ratio
            adjusted_price *= (float(from_quantity) / float(to_quantity))
    return adjusted_price


def merge_dates(left, right):
    sorted_dates = []
    i = 0
    j = 0
    while i < len(left) and j < len(right):
        left_date = datetime.strptime(left[i], "%Y-%m-%d").date()
        right_date = datetime.strptime(right[j], "%Y-%m-%d").date()
        if left_date < right_date:
            sorted_dates.append(left[i])
            i += 1
        else:
            sorted_dates.append(right[j])
            j += 1

    sorted_dates.extend(left[i:])
    sorted_dates.extend(right[j:])
    return sorted_dates


def sort_dates(dates):
    # Merge sort !
    if len(dates) <= 1:
        return dates

    middle = len(dates) // 2
    left = sort_dates(dates[:middle])
    right = sort_dates(dates[middle:])

    return merge_dates(left, right)

def get_last_close_date(ticker, start_date):
    # Dict is already sorted from input but sorting for case it isn't
    # Binary search to find it!
    all_dates = sort_dates(list(price_data[ticker].keys()))
    left = 0
    right = len(all_dates) - 1

    last_close_date = None
    while left <= right:
        middle = left + ((right - left) // 2)
        date_to_check = datetime.strptime(all_dates[middle], "%Y-%m-%d").date()
        if date_to_check > start_date:
            right = middle - 1
        elif date_to_check < start_date:
            last_close_date = date_to_check
            left = middle + 1
        else:
            return None
    return last_close_date


def get_invest_return(ticker_prices, customer_id):
    contribution_cost_total = 0
    contribution_purchases = defaultdict(int)
    start_portfolio_total = 0
    start_portfolio = defaultdict(int)
    current_portfolio_total = 0
    current_portfolio = defaultdict(int)

    for ticker, purchases in portfolio_data[customer_id].items():
        end_price = ticker_prices[ticker]['end_price']
        start_price = ticker_prices[ticker]['start_price']
        start_date = ticker_prices[ticker]['start_date']
        end_date = ticker_prices[ticker]['end_date']

        for purchase in purchases:
            purchase_date = purchase['purchase_date']
            shares_qty = purchase['shares_qty']
            cost_basis = purchase['cost_basis']

            # did the customer own the shares before or on the period start date
            if datetime.strptime(purchase_date, "%Y-%m-%d").date() <= start_date:
                start_value = start_price * int(shares_qty)
                start_portfolio[ticker] += start_value
                start_portfolio_total += start_value
            # did the customer make any contributions during this period to exclude from return
            elif start_date < datetime.strptime(purchase_date, "%Y-%m-%d").date() <= end_date:
                contribution_cost = float(cost_basis) * int(shares_qty)
                contribution_purchases[ticker] += contribution_cost
                contribution_cost_total += contribution_cost

            # add up current value at end of period
            current_value = end_price * int(shares_qty)
            current_portfolio[ticker] += current_value
            current_portfolio_total += current_value

    print(f"Start position: ${start_portfolio_total:.2f}")
    print(f"Current position: ${current_portfolio_total:.2f}")
    print(f"Contributions made during period: ${contribution_cost_total:.2f}")

    # handle customer with $0 at start of period
    if start_portfolio_total == 0:
        if contribution_cost_total > 0:
            investment_return_dollar = (current_portfolio_total - contribution_cost_total)
            investment_return_percentage = ( investment_return_dollar / contribution_cost_total) * 100
        else:
            investment_return_dollar = 0
            investment_return_percentage = 0
    else:
        investment_return_dollar = current_portfolio_total - start_portfolio_total - contribution_cost_total
        investment_return_percentage = (investment_return_dollar / start_portfolio_total) * 100
    sign = "+" if investment_return_dollar > 0 else ""
    print(f"Overall return: ${sign}{investment_return_dollar:.2f} ({sign}{investment_return_percentage:.2f}%)")


def get_ticker_prices_for_timeframe(customer_id, timeframe):
    end_date = date(2024, 12, 31)
    start_date = end_date - timedelta(days=TIMEFRAMES[timeframe])
    customer_data = portfolio_data[customer_id]

    ticker_prices = {}
    for ticker in customer_data:
        aka_tickers = []
        # Handle potential ticker changes
        if ticker in ticker_changes_data:
            aka_tickers = handle_ticker_change(ticker)
        aka_tickers.append(ticker)
        # When date is a weekend or holiday, find last close price
        if start_date.strftime("%Y-%m-%d") not in price_data[ticker]:
            start_date = get_last_close_date(ticker, start_date)
            if not start_date:
                print(f"Requested period for {ticker} not found")
                sys.exit(1)

        end_date_str = end_date.strftime("%Y-%m-%d")
        start_date_str = start_date.strftime("%Y-%m-%d")

        end_close_price = float(price_data[ticker][end_date_str])
        start_close_price = float(price_data[ticker][start_date_str])

        # Handle split if one has occurred during period
        for aka_ticker in aka_tickers:
            if aka_ticker in splits_data:
                start_close_price = handle_split_calculation(aka_ticker, start_date,
                                                            end_date, start_close_price)
        ticker_prices[ticker] =  {
            "start_price": start_close_price,
            "end_price": end_close_price,
            "start_date": start_date,
            "end_date": end_date
        }

    return ticker_prices


def read_ticker_changes_input():
    data = {}
    with open("ticker_changes.csv", encoding="utf-8") as ticker_changes_file:
        reader = csv.DictReader(ticker_changes_file)
        for row in reader:
            old_ticker = row["old_ticker"]
            effective_date = row["effective_date"]
            new_ticker = row["new_ticker"]

            if old_ticker not in data:
                data[old_ticker] = []
            data[old_ticker].append([effective_date, new_ticker])

            if new_ticker not in data:
                data[new_ticker] = []
            data[new_ticker].append([effective_date, old_ticker])
        return data


def read_splits_input():
    data = {}
    with open("splits.csv", encoding="utf-8") as splits_file:
        reader = csv.DictReader(splits_file)
        for row in reader:
            ticker = row["ticker"]
            effective_date = row["effective_date"]
            from_quantity = row["from_quantity"]
            to_quantity = row["to_quantity"]

            if ticker not in data:
                data[ticker] = {}
            data[ticker][effective_date] = [from_quantity, to_quantity]
        return data


def read_price_input():
    data = {}
    with open("prices.csv", encoding="utf-8") as price_file:
        reader = csv.DictReader(price_file)
        for row in reader:
            ticker = row["ticker"]
            close_date = row["date"]
            close_price = row["close_price"]

            if ticker not in data:
                data[ticker] = {}
            data[ticker][close_date] = close_price
        return data


def read_portfolio_input():
    data = {}
    with open("portfolios.csv", encoding="utf-8") as portfolio_file:
        reader = csv.DictReader(portfolio_file)
        for row in reader:
            customer_id = row["customer_id"]
            ticker = row["ticker"]
            purchase_date = row["purchase_date"]
            shares_qty = row["shares"]
            cost_basis = row["cost_basis"]

            if customer_id not in data:
                data[customer_id] = {}
            if ticker not in data[customer_id]:
                data[customer_id][ticker] = []

            data[customer_id][ticker].append({
                "purchase_date": purchase_date,
                "shares_qty": shares_qty,
                "cost_basis": cost_basis
            })

        return data


def valid_arguments(customer_id, timeframe):
    return len(sys.argv) == 3 and customer_id in portfolio_data and timeframe in TIMEFRAMES


if __name__ == "__main__":
    price_data = read_price_input()
    splits_data = read_splits_input()
    ticker_changes_data = read_ticker_changes_input()
    portfolio_data= read_portfolio_input()
    arg_customer = sys.argv[1]
    arg_timeframe = sys.argv[2]

    if not valid_arguments(arg_customer, arg_timeframe):
        print("Usage: returns.py <customer_id> <'timeframe'>")
        print("Example: returns.py CUST001 '6 months'")
        sys.exit(1)

    print(f"{arg_customer} investment return over {arg_timeframe}")

    prices = get_ticker_prices_for_timeframe(arg_customer, arg_timeframe)
    get_invest_return(prices, arg_customer)

# used for unit testing, wouldn't normally use global like that
def unittest_setup():
    global price_data
    global splits_data
    global ticker_changes_data
    price_data = None
    splits_data = None
    ticker_changes_data = None
    price_data = read_price_input()
    splits_data = read_splits_input()
    ticker_changes_data = read_ticker_changes_input()
    