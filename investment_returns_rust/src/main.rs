mod readers;
mod models;
mod logic;

use std::error::Error;
use std::process;
use clap::Parser;
use models::Data;
use models::Args;

fn valid_arguments(data: &Data, args: &Args) -> bool {
    let valid_customer = data.portfolios.contains_key(&args.customer_id);
    let valid_timeframe = logic::get_timeframe_days(&args.timeframe).is_some();
    valid_customer && valid_timeframe
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut data = Data {
        prices: readers::read_prices()?,
        splits: readers::read_splits()?,
        ticker_changes: readers::read_ticker_changes()?,
        portfolios: readers::read_portfolios()?,
    };

    let args = Args::parse();
    if !valid_arguments(&data, &args) {
        println!("Usage: cargo run <customer_id> <timeframe>");
        println!("Example: cargo run CUST001 \"6 months\"");
        process::exit(1);
    }
    println!("{} investment return over {}", args.customer_id, args.timeframe); 

    let ticker_prices = logic::get_ticker_prices_for_timeframe(&mut data, &args)?;

    let customer_data = data.portfolios.get(&args.customer_id).unwrap();
    let return_total = logic::get_invest_return(&ticker_prices, customer_data);
    logic::output_result(return_total);

    Ok(())
}