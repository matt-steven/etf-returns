## Portfolio Investment Returns
A python and a rust solution to this problem:

Takes a customer id and time period such as '1 year'. 
The customer's portfolio postion at the start date and end date of the period is calcuated.
Any contributions made during the period are excluded from the end return amount. 
But any gains or losses from these purchases will effect the end return.
The end result is purely the performance of the portfolio's holdings.
Ticker changes and splits are accounted for in the calculation.

![screenshot](/example_cli_use3.png)
```bash
cd investment_returns_rust

cargo build

cargo run CUST004 '1 year'
```

![screenshot](/example_cli_use2.png)
```bash
python investment_returns.py CUST002 '1 year'
```

## ETF Returns

Takes a ticker and a time period such as '6 months' as input.
The start price and end price of the period for the ticker are compared and the performance is output.
Ticker changes and splits are accounted for in the prices.

![screenshot](/example_cli_use.png)
```bash
python returns.py NDQ '6 months'
```

## Unit Tests

Some example unit tests are also included.
```bash
python tests.py
```