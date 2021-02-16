#![macro_use]
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate chrono;
extern crate csv;
extern crate reqwest;

use reqwest::blocking::Client;
use chrono::NaiveDateTime;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug)]
pub struct CandleRequestArgs {
    pub pair: String,
    pub timeframe: u64,
    pub start_utc: i64,
    pub end_utc: i64
}

impl Default for CandleRequestArgs {
    fn default() -> CandleRequestArgs {
        CandleRequestArgs {
            pair: String::from("USDT_BTC"),
            timeframe: 300,
            start_utc: NaiveDateTime::parse_from_str("2020-6-1 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap().timestamp(),
            end_utc: NaiveDateTime::parse_from_str("2020-7-1 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap().timestamp(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CandleData {
    pub date: u64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub close: f64,
    pub volume: f64,
    pub quoteVolume: f64,
    pub weightedAverage: f64,
}

pub fn datetime_string_to_timestamp(time_string: String) -> i64 {
    NaiveDateTime::parse_from_str(&time_string, "%Y-%m-%d %H:%M:%S").unwrap().timestamp()
}

pub fn download_candles(candle_request_args: &CandleRequestArgs) -> Vec<CandleData> {
    let client = Client::new();
        
    let url: &str= &(format!("https://poloniex.com/public?command=returnChartData&currencyPair={0}&start={1}&end={2}&resolution=auto&period={3}",
                            candle_request_args.pair, candle_request_args.start_utc, candle_request_args.end_utc, candle_request_args.timeframe));
    let body_text = match client.get(url).send() {
        Ok(resp) => match resp.text() {
            Ok(text) => {
                if text.contains("error") {
                    panic!(text)
                }
                text
            },
            Err(_ee) => panic!("Got nothing from poloniex")
        },
        Err(e) => panic!("Failed web requeest: {}", e)
    };

    let json_array: Vec<CandleData> = serde_json::from_str(&body_text).expect("Json parsing error");
    println!("Records downloaded {}", json_array.len());

    json_array    
}
