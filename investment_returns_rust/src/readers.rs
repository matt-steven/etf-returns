use std::collections::HashMap;
use std::error::Error;
use csv::Reader;

use crate::models::PriceRecord;
use crate::models::Split;
use crate::models::TickerChanges;
use crate::models::Portfolio;
use crate::models::PortfolioPurchase;

pub fn read_prices() -> Result<HashMap<String, HashMap<String, f64>>, Box<dyn Error>> {
    let mut prices: HashMap<String, HashMap<String, f64>> = HashMap::new();
    let mut rdr = Reader::from_path("../prices.csv")?;
    
    for result in rdr.deserialize() {
        let record: PriceRecord = result?;

        prices
            .entry(record.ticker)
            .or_insert_with(HashMap::new)
            .insert(record.date, record.close_price);
    }
    
    Ok(prices)
}

pub fn read_splits() -> Result<HashMap<String, HashMap<String, [f64; 2]>>, Box<dyn Error>> {
    let mut splits: HashMap<String, HashMap<String, [f64; 2]>> = HashMap::new();
    let mut rdr = Reader::from_path("../splits.csv")?;
    
    for result in rdr.deserialize() {
        let record: Split = result?;

        splits
            .entry(record.ticker)
            .or_insert_with(HashMap::new)
            .insert(record.effective_date, [record.from_quantity, record.to_quantity]);
    }
    
    Ok(splits)
}

pub fn read_ticker_changes() -> Result<HashMap<String, Vec<Vec<String>>>, Box<dyn Error>> {
    let mut ticker_changes: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    let mut rdr = Reader::from_path("../ticker_changes.csv")?;
    
    for result in rdr.deserialize() {
        let record: TickerChanges = result?;

        ticker_changes
            .entry(record.old_ticker.clone())
            .or_insert_with(Vec::new)
            .push(vec![record.effective_date.clone(), record.new_ticker.clone()]);
        ticker_changes
            .entry(record.new_ticker)
            .or_insert_with(Vec::new)
            .push(vec![record.effective_date, record.old_ticker]);
    }
    
    Ok(ticker_changes)
}

pub fn read_portfolios() -> Result<HashMap<String, HashMap<String, Vec<PortfolioPurchase>>>, Box<dyn Error>> {
    let mut portfolios: HashMap<String, HashMap<String, Vec<PortfolioPurchase>>> = HashMap::new();
    let mut rdr = Reader::from_path("../portfolios.csv")?;
    
    for result in rdr.deserialize() {
        let record: Portfolio = result?;

        portfolios
            .entry(record.customer_id)
            .or_insert_with(HashMap::new)
            .entry(record.ticker)
            .or_insert_with(Vec::new)
            .push(PortfolioPurchase {
                purchase_date: record.purchase_date,
                shares_qty: record.shares,
                cost_basis: record.cost_basis,
            });
    }
    
    Ok(portfolios)
}