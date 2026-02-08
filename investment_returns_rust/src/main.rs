mod readers;
mod models;
mod logic;

use std::error::Error;
use clap::Parser;
use models::Data;
use models::Args;

fn main() -> Result<(), Box<dyn Error>> {
    let mut data = Data {
        prices: readers::read_prices()?,
        splits: readers::read_splits()?,
        ticker_changes: readers::read_ticker_changes()?,
        portfolios: readers::read_portfolios()?,
    };

    let args = Args::parse();
    println!("{} investment return over {}", args.customer_id, args.timeframe); 

    let ticker_prices = logic::get_ticker_prices_for_timeframe(&mut data, &args);

    // let return_total = get_invest_return(ticker_prices, args.customer_id);
    // output_result(return_total);

    Ok(())
}