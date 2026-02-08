use std::error::Error;
use std::collections::HashMap;
use chrono::{NaiveDate, Duration};

use crate::models::Data;
use crate::models::Args;
use crate::models::TickerPrice;
use crate::models::PortfolioPurchase;
use crate::models::PortfolioReturn;


pub fn get_timeframe_days(timeframe_str: &str) -> Option<i64> {
    match timeframe_str {
        "1 day" => Some(1),
        "5 days" => Some(5),
        "6 months" => Some(182),
        "1 year" => Some(365),
        _ => None,
    }
}

fn get_last_close_date(
    ticker_prices: &HashMap<String, f64>, 
    start_date: NaiveDate
) -> Option<NaiveDate> {
    let mut last_date: Option<NaiveDate> = None;

    for date_str in ticker_prices.keys() {
        if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            if date < start_date {
                if last_date.is_none() || date > last_date.unwrap() {
                    last_date = Some(date);
                }
            }
        }
    }
    last_date
}

fn handle_split_customer_position(
    purchases: &mut Vec<PortfolioPurchase>, 
    splits: &HashMap<String, [f64; 2]>, 
    end_date: NaiveDate
) {
    for purchase in purchases.iter_mut() {
        for (split_date_str, split_ratio) in splits {
            let split_date = NaiveDate::parse_from_str(split_date_str, "%d/%m/%Y").unwrap();
            let purchase_date = NaiveDate::parse_from_str(&purchase.purchase_date, "%Y-%m-%d").unwrap();

            if purchase_date < split_date && split_date <= end_date {
                let split_multiplier = split_ratio[1] / split_ratio[0];
                purchase.shares_qty *= split_multiplier;
                purchase.cost_basis /= split_multiplier;
            }
        }
    }
}

fn handle_split_price( splits: &HashMap<String, [f64; 2]>, start_date: NaiveDate, end_date: NaiveDate, mut price: f64) -> f64 {
    for (split_date_str, split_ratio) in splits {
        let split_date = NaiveDate::parse_from_str(split_date_str, "%d/%m/%Y").unwrap();

        if start_date < split_date && split_date <= end_date {
            let from_quantity = split_ratio[0];
            let to_quantity = split_ratio[1];
            price *= from_quantity / to_quantity;
        }
    }
    price
}

fn handle_ticker_change(
    ticker_changes: &HashMap<String, Vec<Vec<String>>>,
    prices: &mut HashMap<String, HashMap<String, f64>>,
    ticker: &str,
) -> Vec<String> {
    let mut changed_tickers = Vec::new();

    if let Some(changes) = ticker_changes.get(ticker).cloned() {
        for change in changes {
            let changed_ticker = &change[1];

            if let Some(changed_prices) = prices.get(changed_ticker).cloned() {
                if let Some(ticker_prices) = prices.get_mut(ticker) {
                    for (date, price) in changed_prices {
                        ticker_prices.insert(date, price);
                    }
                }
            }
            changed_tickers.push(changed_ticker.clone());
        }
    }
    changed_tickers
}

pub fn output_result(portfolio_return: PortfolioReturn) {
    let start_portfolio_total = portfolio_return.start_total;
    let current_portfolio_total = portfolio_return.current_total;
    let contribution_cost_total = portfolio_return.contribution_total;

    println!("Start position: ${:.2}", start_portfolio_total);
    println!("Current position: ${:.2}", current_portfolio_total);
    println!("Contributions made during period: ${:.2}", contribution_cost_total);

    let (investment_return_dollar, investment_return_percentage) = if start_portfolio_total == 0.0 {
        if contribution_cost_total > 0.0 {
            let dollar = current_portfolio_total - contribution_cost_total;
            let percentage = (dollar / contribution_cost_total) * 100.0;
            (dollar, percentage)
        } else {
            (0.0, 0.0)
        }
    } else {
        let dollar = current_portfolio_total - start_portfolio_total - contribution_cost_total;
        let percentage = (dollar / start_portfolio_total) * 100.0;
        (dollar, percentage)
    };

    let sign = if investment_return_dollar > 0.0 {
        "+"
    } else {
        ""
    };

    println!("Overall return: ${}{:.2} ({}{:.2}%)", sign, investment_return_dollar, sign, investment_return_percentage);
}

pub fn get_invest_return(
    ticker_prices: &HashMap<String, TickerPrice>,
    customer_data: &HashMap<String, Vec<PortfolioPurchase>>
) -> PortfolioReturn {
    let mut contribution_cost_total = 0.0;
    let mut start_portfolio_total = 0.0;
    let mut current_portfolio_total = 0.0;

    for (ticker, purchases) in customer_data {
        if let Some(price_info) = ticker_prices.get(ticker) {
            let end_price = price_info.end_price;
            let start_price = price_info.start_price;
            let start_date = NaiveDate::parse_from_str(&price_info.start_date, "%Y-%m-%d").unwrap();
            let end_date = NaiveDate::parse_from_str(&price_info.end_date, "%Y-%m-%d").unwrap();

            for purchase in purchases {
                let purchase_date = NaiveDate::parse_from_str(&purchase.purchase_date, "%Y-%m-%d").unwrap();
                let shares_qty = purchase.shares_qty;
                let cost_basis = purchase.cost_basis;

                if purchase_date <= start_date {
                    let start_value = start_price * shares_qty;
                    start_portfolio_total += start_value;
                } else if purchase_date > start_date && purchase_date <= end_date {
                    let contribution_cost = cost_basis * shares_qty;
                    contribution_cost_total += contribution_cost;
                }

                let current_value = end_price * shares_qty;
                current_portfolio_total += current_value;
            }
        }
    }

    PortfolioReturn {
        start_total: start_portfolio_total,
        current_total: current_portfolio_total,
        contribution_total: contribution_cost_total,
    }
}

pub fn get_ticker_prices_for_timeframe(
    data: &mut Data, 
    args: &Args
) -> Result<HashMap<String, TickerPrice>, Box<dyn Error>> {
    let mut ticker_prices: HashMap<String, TickerPrice> = HashMap::new();
    let end_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    let no_of_days = get_timeframe_days(&args.timeframe).expect("Invalid timeframe");
    let start_date = end_date - Duration::days(no_of_days);
    
    let customer_data = data.portfolios
        .get_mut(&args.customer_id)
        .ok_or("Customer not found")?;

    for (ticker, purchases) in customer_data.iter_mut() {
        let mut aka_tickers: Vec<String> = Vec::new();
        let mut ticker_start_date = start_date;

        // Ticker change handling
         if data.ticker_changes.contains_key(ticker) {
            aka_tickers = handle_ticker_change(&data.ticker_changes, &mut data.prices, ticker);
        }
        aka_tickers.push(ticker.clone());

        let start_date_str = ticker_start_date.format("%Y-%m-%d").to_string();
        if let Some(ticker_price_data) = data.prices.get(ticker) {
            // Find last close price if current one is weekend or holiday
            if !ticker_price_data.contains_key(&start_date_str) {
                ticker_start_date = get_last_close_date(ticker_price_data, ticker_start_date)
                .ok_or("Requested period not found for ticker")?;
            }

            let end_date_str = end_date.format("%Y-%m-%d").to_string();
            let start_date_str = ticker_start_date.format("%Y-%m-%d").to_string();
            let end_close_price = ticker_price_data[&end_date_str];
            let mut start_close_price = ticker_price_data[&start_date_str];

            // Split handling
            for aka_ticker in &aka_tickers {
                if let Some(splits) = data.splits.get(aka_ticker) {
                    start_close_price = handle_split_price(splits, ticker_start_date, end_date, start_close_price);
                    handle_split_customer_position(purchases, splits, end_date);
                }
            }

            ticker_prices.insert(
                ticker.clone(),
                TickerPrice {
                    start_price: start_close_price,
                    end_price: end_close_price,
                    start_date: start_date_str,
                    end_date: end_date_str,
                },
            );
        }
    }
    Ok(ticker_prices)
}