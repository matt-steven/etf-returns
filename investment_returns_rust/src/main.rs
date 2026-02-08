mod readers;
mod models;

use clap::Parser;

#[derive(Parser)]
struct Args {
    customer_id: String,
    timeframe: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let prices = readers::read_prices()?;
    let splits = readers::read_splits()?;
    let ticker_changes = readers::read_ticker_changes()?;
    let portfolios = readers::read_portfolios()?;


    println!("Hello, {} {}", args.customer_id, args.timeframe);
    println!("{:#?}", prices);
    println!("{:#?}", splits);
    println!("{:#?}", ticker_changes);
    println!("{:#?}", portfolios);

    Ok(())
}