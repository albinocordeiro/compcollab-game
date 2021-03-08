use color_eyre::{Report, Result, eyre::eyre};
use reqwest::blocking::Client;
use chrono::NaiveDateTime;
use serde_derive::{Serialize, Deserialize};
use crate::data::model::NewCandle;
use crate::data::dbapi::insert_candles;

#[derive(Debug, Copy, Clone)]
pub struct CandleRequestArgs<'a> {
    pub pair: &'a str,
    pub timeframe: u64,
    pub start_utc: i64,
    pub end_utc: i64
}

impl<'a> Default for CandleRequestArgs<'a> {
    fn default() -> CandleRequestArgs<'a> {
        CandleRequestArgs {
            pair: "USDT_BTC",
            timeframe: 300,
            start_utc: NaiveDateTime::parse_from_str("2020-6-1 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap().timestamp(),
            end_utc: NaiveDateTime::parse_from_str("2020-7-1 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap().timestamp(),
        }
    }
}

impl<'a> CandleRequestArgs<'a> {
    fn get_output_file_name(&self) -> Result<String> {
        let mut res = String::from("");
        res.push_str(self.pair);
        res.push_str("_");
        res.push_str(&self.timeframe.to_string());
        res.push_str("_");
        res.push_str(&self.start_utc.to_string());
        res.push_str("_");
        res.push_str(&self.end_utc.to_string());
        res.push_str(".csv");
        
        Ok(res)
    }

    fn get_table_name(self) -> Result<String> {
        Ok(self.pair.to_lowercase())
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PoloniexCandle {
    pub date: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub quote_volume: f64,
    pub weighted_average: f64,
}

impl PoloniexCandle {
    fn get_generic_candle(self) -> NewCandle {   
        NewCandle {
            date: NaiveDateTime::from_timestamp(self.date as i64, 0),
            open: self.open,
            high: self.high,
            low: self.low,
            close: self.close,
            volume: self.volume,
        }
    }
}

pub fn datetime_string_to_timestamp(time_string: String) -> i64 {
    NaiveDateTime::parse_from_str(&time_string, "%Y-%m-%d %H:%M:%S").unwrap().timestamp()
}

pub fn download_candles(candle_request_args: &CandleRequestArgs) -> Result<()> {
    let client = Client::new();
        
    let url: &str= &(format!("https://poloniex.com/public?command=returnChartData&currencyPair={0}&start={1}&end={2}&resolution=auto&period={3}",
                            candle_request_args.pair, candle_request_args.start_utc, candle_request_args.end_utc, candle_request_args.timeframe));

    let body_text = match client.get(url).send() {
        Ok(resp) => match resp.text() {
            Ok(text) => {
                if text.contains("error") {
                    return Err(eyre!(text));
                } else {
                    text
                }
            },
            Err(ee) => return Err(Report::from(ee))
        },
        Err(e) => return Err(Report::from(e))
    };

    let poloniex_candles: Vec<PoloniexCandle> = serde_json::from_str(&body_text)?;

    println!("Records downloaded {}", poloniex_candles.len());
    if poloniex_candles.len() <= 0 {
        return Err(eyre!("Download request returned zero candles."));
    }
    let mut new_candles = Vec::new();
    for pol_candle in &poloniex_candles {
        let new_candle: NewCandle = pol_candle.get_generic_candle();
        new_candles.push(new_candle);
    }
    
    let csv_file_name = candle_request_args.get_output_file_name()?;
    let table_name = candle_request_args.get_table_name()?;

    save_candles_to_csv(&csv_file_name, &new_candles)?;
    push_candles_to_database(&table_name, &new_candles)?;
    
    Ok(())
}

fn save_candles_to_csv(csv_file_name: &str, candles: &[NewCandle]) -> Result<()> {
    use csv::Writer;
    let mut csvwriter = Writer::from_path(csv_file_name)?;

    for candle in candles {
        csvwriter.serialize(&candle)?;
    }
    
    csvwriter.flush()?;

    Ok(())
}

fn push_candles_to_database(table_name: &str, candles: &[NewCandle]) -> Result<()> {
    insert_candles(table_name, candles)?;
    Ok(())
}