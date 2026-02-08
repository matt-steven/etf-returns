use serde::Deserialize;

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
pub struct PortfolioPurchase {
    pub purchase_date: String,
    pub shares_qty: f64,
    pub cost_basis: f64,
}
