extern crate csv;
extern crate diesel::

use std::io;
use csv::Reader as CsvReader;

use super::sources::poloniex::CandleData as PoloniexCandle;

#[derive(AsChangeset, Debug, Serialize, Deserialize)]

pub struct Candle {
    #[]
    pub id: u64,
    pub ts: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64
}

pub fn push_poloniex_csv_to_db(filepath: &str) -> Result<(), io::Error>{
    let mut rdr = CsvReader::from_path(filepath)?;
    for row in rdr.deserialize() {
        let poloniex_candle: PoloniexCandle = row?;
        let candle = poloniex_candle_to_candle(poloniex_candle);
        candle.
    }
    Ok(())
}
