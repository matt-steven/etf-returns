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
        let ticker_start_date = start_date;

        // Ticker change handling
         if data.ticker_changes.contains_key(ticker) {
            aka_tickers = handle_ticker_change(&data.ticker_changes,&mut data.prices, ticker);
        }
        aka_tickers.push(ticker.clone());

        println!("{:?}", aka_tickers)

    }


    Ok(ticker_prices)
}