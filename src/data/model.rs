use serde_derive::{Serialize, Deserialize};
use chrono;


#[derive(Debug, Serialize, Clone, Copy, Deserialize)]
pub struct NewCandle {
    pub date: chrono::NaiveDateTime,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Queryable)]
pub struct Candle {
    pub id: u64,
    pub date: chrono::NaiveDateTime,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}