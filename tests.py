"""
Example tests for ETF returns functions
Also includes tests for Investment returns functions
Usage: python -m unittest tests.py
"""

import unittest

from returns import get_prices_for_period, calc_price_return, sort_dates, unittest_setup
from investment_returns import get_ticker_prices_for_timeframe, get_invest_return, unittest_setup as investment_unittest_setup

class TestReturns(unittest.TestCase):

    def test_five_day_return(self):
        unittest_setup()
        ticker = "ETHI"
        timeframe = "5 days"
        expected_start_price = 16.18
        expected_end_price = 16.10

        actual_end_price, actual_start_price = get_prices_for_period(ticker, timeframe)
        # Test the correct close_prices
        self.assertListEqual([actual_start_price, actual_end_price], [expected_start_price, expected_end_price])

        expected_price_return = ((float(expected_end_price) / float(expected_start_price)) - 1) * 100
        actual_price_return = calc_price_return(actual_end_price, actual_start_price)
        self.assertEqual(actual_price_return, expected_price_return) # Test return calculation is accurate


    def test_get_prices(self):
        unittest_setup()
        ticker = "NDQ"
        timeframe = "6 months"
        expected_start_price = 44.24
        expected_end_price = 50.75

        actual_end_price, actual_start_price = get_prices_for_period(ticker, timeframe)
        # Test the correct close_prices
        self.assertListEqual([actual_start_price, actual_end_price], [expected_start_price, expected_end_price])

        expected_price_return = ((float(expected_end_price) / float(expected_start_price)) - 1) * 100
        actual_price_return = calc_price_return(actual_end_price, actual_start_price)
        self.assertEqual(actual_price_return, expected_price_return) # Test return calculation is accurate


    def test_ticker_change(self):
        unittest_setup()
        ticker = "CYBR"
        timeframe = "1 year"
        expected_start_price = 10.83
        expected_end_price = 14.02

        actual_end_price, actual_start_price = get_prices_for_period(ticker, timeframe)
        # Test the correct close_prices
        self.assertListEqual([actual_start_price, actual_end_price], [expected_start_price, expected_end_price])

        expected_price_return = ((float(expected_end_price) / float(expected_start_price)) - 1) * 100
        actual_price_return = calc_price_return(actual_end_price, actual_start_price)
        self.assertEqual(actual_price_return, expected_price_return) # Test return calculation is accurate


    def test_split(self):
        unittest_setup()
        ticker = "A123"
        timeframe = "1 year"
        expected_start_price_after_split = 126.93 / 5
        expected_end_price = 27.42

        actual_end_price, actual_start_price = get_prices_for_period(ticker, timeframe)
        # Test the correct close_prices
        self.assertListEqual([actual_start_price, actual_end_price], [expected_start_price_after_split, expected_end_price])

        expected_price_return = ((float(expected_end_price) / float(expected_start_price_after_split)) - 1) * 100
        actual_price_return = calc_price_return(actual_end_price, actual_start_price)
        self.assertEqual(actual_price_return, expected_price_return) # Test return calculation is accurate


    def test_sort(self):
        test_list = ["2024-06-07", "2024-05-30", "2024-08-15", "2024-08-20", "2023-12-27", "2023-12-13", "2024-06-28"]
        expected = ["2023-12-13", "2023-12-27", "2024-05-30", "2024-06-07", "2024-06-28", "2024-08-15", "2024-08-20"]
        actual = sort_dates(test_list)
        self.assertEqual(actual, expected) # Test sort works with odd number of elements, this passes :)

        expected = ["2023-12-13", "2023-12-27", "2024-05-30", "2024-06-07", "2024-06-28", "2024-08-15", "2024-08-20", "2025-02-03"]
        test_list = ["2024-06-07", "2024-05-30", "2024-08-15", "2024-08-20", "2025-02-03", "2023-12-27", "2023-12-13", "2024-06-28"]
        actual = sort_dates(test_list)
        self.assertEqual(actual, expected) # Test sort works with even number of elements, this passes :)

        # expected = ["2023-12-13", "2023-12-27", "2024-05-30", "2024-06-07", "2024-06-28", "2024-08-15", "2024-08-20"]
        # test_list = ["2024/06/07", "2024-05-30", "2024-08-15", "2024/08/20", "2023-12-27", "2023-12-13", "2024-06-28"]
        # actual = sort_dates(test_list)
        # self.assertEqual(actual, expected) # Test different date formats, this fails :(

        test_list = ["2024-06-07"]
        expected = ["2024-06-07"]
        actual = sort_dates(test_list)
        self.assertEqual(actual, expected) # Test single element list, this passes :)

        test_list = []
        expected = []
        actual = sort_dates(test_list)
        self.assertEqual(actual, expected) # Test empty list, this passes :)

    def test_portfolio_return(self):
        """
        - Customer bought 10 shares of TEST at $100 each on 2024-01-01
        - Stock price went from $100 to $110 over 1 year
        - Expected return: $100 gain, or 10%
        """
        import investment_returns

        # Reset
        investment_returns.price_data = {}
        investment_returns.splits_data = {}
        investment_returns.ticker_changes_data = {}
        investment_returns.portfolio_data = {}

        # Test data
        investment_returns.price_data = {
            "TEST": {
                "2023-12-31": "100",
                "2024-12-31": "110",
            }
        }
        investment_returns.splits_data = {}
        investment_returns.ticker_changes_data = {}
        investment_returns.portfolio_data = {
            "TEST001": {
                "TEST": [
                    {
                        "purchase_date": "2023-01-01",
                        "shares_qty": "10",
                        "cost_basis": "100",
                    }
                ]
            }
        }

        customer_id = "TEST001"
        timeframe = "1 year"

        ticker_prices = get_ticker_prices_for_timeframe(customer_id, timeframe)
        return_total = get_invest_return(ticker_prices, customer_id)

        self.assertAlmostEqual(return_total["start_total"], 1000.0, places=2) # Customer had 10 shares at start price of $100 = $1000 start position
        self.assertAlmostEqual(return_total["current_total"], 1100.0, places=2) # Customer has 10 shares at end price of $110 = $1100 current position
        self.assertAlmostEqual(return_total["contribution_total"], 0.0, places=2) # No contributions during the period
        
        expected_dollar_return = 100.0 # Calculate expected return: $1100 - $1000 = $100 gain
        actual_dollar_return = return_total["current_total"] - return_total["start_total"] - return_total["contribution_total"]
        self.assertAlmostEqual(actual_dollar_return, expected_dollar_return, places=2)

    def test_portfolio_return_with_contributions(self):
        """
        - Customer had 10 shares at start at $100
        - Customer bought 5 more shares during period at $105
        - Stock price ends at $110
        """
        import investment_returns

        # Reset
        investment_returns.price_data = {}
        investment_returns.splits_data = {}
        investment_returns.ticker_changes_data = {}
        investment_returns.portfolio_data = {}

        # Test data
        investment_returns.price_data = {
            "TEST": {
                "2023-12-31": "100",
                "2024-12-31": "110",
            }
        }
        investment_returns.splits_data = {}
        investment_returns.ticker_changes_data = {}
        investment_returns.portfolio_data = {
            "TEST002": {
                "TEST": [
                    {
                        "purchase_date": "2023-06-01",
                        "shares_qty": "10",
                        "cost_basis": "100",
                    },
                    {
                        "purchase_date": "2024-06-01",
                        "shares_qty": "5",
                        "cost_basis": "105",
                    },
                ]
            }
        }

        customer_id = "TEST002"
        timeframe = "1 year"

        ticker_prices = get_ticker_prices_for_timeframe(customer_id, timeframe)
        return_total = get_invest_return(ticker_prices, customer_id)

        self.assertAlmostEqual(return_total["start_total"], 1000.0, places=2) # Start position: 10 shares at $100 = $1000
        self.assertAlmostEqual(return_total["contribution_total"], 525.0, places=2) # Contributions: 5 shares at $105 = $525
        self.assertAlmostEqual(return_total["current_total"], 1650.0, places=2) # Current position: 15 shares at $110 = $1650

        expected_dollar_return = 125.0 # Return: $1650 - $1000 - $525 = $125
        actual_dollar_return = return_total["current_total"] - return_total["start_total"] - return_total["contribution_total"]
        self.assertAlmostEqual(actual_dollar_return, expected_dollar_return, places=2)

    def test_portfolio_return_with_split(self):
        """
        - Customer had 10 shares at start at $100
        - 2 to 1 split occurs on 2024-06-01
        - After split qty is 20 shares and start price is $50
        - Stock ends at $55
        """
        import investment_returns

        # Reset
        investment_returns.price_data = {}
        investment_returns.splits_data = {}
        investment_returns.ticker_changes_data = {}
        investment_returns.portfolio_data = {}

        # Test data
        investment_returns.price_data = {
            "TEST": {
                "2023-12-31": "100",
                "2024-12-31": "55",
            }
        }
        investment_returns.splits_data = {
            "TEST": {
                "01/06/2024": ["1", "2"], 
            }
        }
        investment_returns.ticker_changes_data = {}
        investment_returns.portfolio_data = {
            "TEST003": {
                "TEST": [
                    {
                        "purchase_date": "2023-06-01",
                        "shares_qty": "10",
                        "cost_basis": "100",
                    }
                ]
            }
        }
        customer_id = "TEST003"
        timeframe = "1 year"

        ticker_prices = get_ticker_prices_for_timeframe(customer_id, timeframe)
        return_total = get_invest_return(ticker_prices, customer_id)
        
        self.assertAlmostEqual(return_total["start_total"], 1000.0, places=2) # Start position: 20 shares (after split) at $50 (adjusted) = $1000
        self.assertAlmostEqual(return_total["current_total"], 1100.0, places=2) # Current position: 20 shares at $55 = $1100
        self.assertAlmostEqual(return_total["contribution_total"], 0.0, places=2) # No contributons
        expected_dollar_return = 100.0 # Return: $1100 - $1000 = $100
        actual_dollar_return = return_total["current_total"] - return_total["start_total"] - return_total["contribution_total"]
        self.assertAlmostEqual(actual_dollar_return, expected_dollar_return, places=2)

    def test_portfolio_return_with_ticker_change(self):
        """
        - Customer holds OLD ticker
        - Ticker changes to NEW on 2024-06-01
        """
        import investment_returns

        # Reset
        investment_returns.price_data = {}
        investment_returns.splits_data = {}
        investment_returns.ticker_changes_data = {}
        investment_returns.portfolio_data = {}

        # Test data
        investment_returns.price_data = {
            "OLD": {
                "2023-12-31": "100",
            },
            "NEW": {
                "2024-12-31": "110",
            },
        }
        investment_returns.splits_data = {}
        investment_returns.ticker_changes_data = {
            "OLD": [["01/06/2024", "NEW"]],
            "NEW": [["01/06/2024", "OLD"]],
        }
        investment_returns.portfolio_data = {
            "TEST004": {
                "OLD": [
                    {
                        "purchase_date": "2023-06-01",
                        "shares_qty": "10",
                        "cost_basis": "100",
                    }
                ]
            }
        }
        customer_id = "TEST004"
        timeframe = "1 year"

        ticker_prices = get_ticker_prices_for_timeframe(customer_id, timeframe)
        return_total = get_invest_return(ticker_prices, customer_id)

        self.assertAlmostEqual(return_total["start_total"], 1000.0, places=2) # Start position: 10 shares at $100 = $1000
        self.assertAlmostEqual(return_total["current_total"], 1100.0, places=2) # Current position: 10 shares at $110 = $1100
        self.assertAlmostEqual(return_total["contribution_total"], 0.0, places=2) # No contributons
   
        expected_dollar_return = 100.0 # Return: $1100 - $1000 = $100
        actual_dollar_return = return_total["current_total"] - return_total["start_total"] - return_total["contribution_total"]
        self.assertAlmostEqual(actual_dollar_return, expected_dollar_return, places=2)


if __name__ == "__main__":
    unittest.main()
