use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Parser)]
pub struct Args {
    pub customer_id: String,
    pub timeframe: String,
}

#[derive(Debug, Deserialize)]
pub struct PriceRecord {
    pub ticker: String,
    pub date: String,
    pub close_price: f64,
}

#[derive(Debug, Deserialize)]
pub struct Split {
    pub ticker: String,
    pub effective_date: String,
    pub from_quantity: f64,
    pub to_quantity: f64,
}

#[derive(Debug, Deserialize)]
pub struct TickerChanges {
    pub old_ticker: String,
    pub new_ticker: String,
    pub effective_date: String,
}

#[derive(Debug, Deserialize)]
pub struct Portfolio {
    pub customer_id: String,
    pub ticker: String,
    pub purchase_date: String,
    pub shares: f64,
    pub cost_basis: f64,
}

#[derive(Debug)]
pub struct Data {
    pub prices: HashMap<String, HashMap<String, f64>>,
    pub splits: HashMap<String, HashMap<String, [f64; 2]>>,
    pub ticker_changes: HashMap<String, Vec<Vec<String>>>,
    pub portfolios: HashMap<String, HashMap<String, Vec<PortfolioPurchase>>>,
}

#[derive(Debug)]
pub struct PortfolioPurchase {
    pub purchase_date: String,
    pub shares_qty: f64,
    pub cost_basis: f64,
}

#[derive(Debug)]
pub struct TickerPrice {
    pub start_price: f64,
    pub end_price: f64,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug)]
pub struct PortfolioReturn {
    pub start_total: f64,
    pub current_total: f64,
    pub contribution_total: f64,
}