use std::error::Error;
use std::collections::HashMap;
use chrono::{NaiveDate, Duration};

use crate::models::Data;
use crate::models::Args;
use crate::models::TickerPrice;

fn get_timeframe_days(timeframe_str: &str) -> Option<i64> {
    match timeframe_str {
        "1 day" => Some(1),
        "5 days" => Some(5),
        "6 months" => Some(182),
        "1 year" => Some(365),
        _ => None,
    }
}

fn get_last_close_date(ticker_prices: &HashMap<String, f64>, start_date: NaiveDate) -> Option<NaiveDate> {
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

pub fn get_ticker_prices_for_timeframe(data: &mut Data, args: &Args) -> Result<HashMap<String, TickerPrice>, Box<dyn Error>> {
    let ticker_prices: HashMap<String, TickerPrice> = HashMap::new();
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

            println!("{:?} - {:?}",end_close_price, start_close_price)
        }
    }

    Ok(ticker_prices)
}