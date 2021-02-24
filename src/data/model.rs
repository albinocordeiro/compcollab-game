use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Clone, Copy, Deserialize)]
pub struct NewCandle {
    pub date: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}
#[derive(Default, Debug, Queryable)]
pub struct Candle {
    pub id: u64,
    pub date: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}