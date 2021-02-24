mod model;

use crate::schema;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::io;
use csv::Reader as CsvReader;
use crate::data::model::Candle;

use crate::data::sources::poloniex::CandleData as PoloniexCandle;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn poloniex_candle_to_candle(pcandle: &PoloniexCandle) -> NewCandle {
    let output: Candle = Candle::default();
    output.ts = pcandle.date;
    output.open = pcandle.open;
    output.close = pcandle.close;
    output.high = pcandle.high;
    output.low = pcandle.low;
    output.volume = pcandle.volume;
    output
}

pub fn push_poloniex_csv_to_db(filepath: &str) -> Result<(), io::Error>{
    let mut rdr = CsvReader::from_path(filepath)?;
    for row in rdr.deserialize() {
        let poloniex_candle: PoloniexCandle = row?;
        
    }
    Ok(())
}
