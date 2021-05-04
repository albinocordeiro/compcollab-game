use super::model::Candle;
use crate::schema::usdt_btc;

use chrono::NaiveDateTime;
use color_eyre::{eyre::eyre, Result};
use diesel::prelude::*;

enum SupportedTable {
    UsdtBtc(usdt_btc::table),
}

impl SupportedTable {
    fn from_table_name(table_name: &str) -> Result<SupportedTable> {
        match table_name {
            "usdt_btc" => Ok(SupportedTable::UsdtBtc(usdt_btc::table)),
            _ => Err(eyre!(
                "A table for the currency pair {} was not found in the database",
                table_name
            )),
        }
    }
}

pub fn get_db_connection() -> Result<PgConnection> {
    use dotenv::dotenv;
    use std::env;
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let conn = PgConnection::establish(&database_url)?;
    Ok(conn)
}

pub fn insert_candles(table_name: &str, candles: &[Candle]) -> Result<()> {
    use diesel::insert_into;
    let supported_table = SupportedTable::from_table_name(table_name)?;
    let (get_table, get_row) = match supported_table {
        SupportedTable::UsdtBtc(_table) => (
            || usdt_btc::table,
            |t: NaiveDateTime, o: f64, h: f64, l: f64, c: f64, v: f64| {
                (
                    usdt_btc::timestamp.eq(t),
                    usdt_btc::open.eq(o),
                    usdt_btc::high.eq(h),
                    usdt_btc::low.eq(l),
                    usdt_btc::close.eq(c),
                    usdt_btc::volume.eq(v),
                )
            },
        ),
    };

    let conn = get_db_connection()?;

    let batch_size = 1000usize;
    let mut row_count = 0usize;
    let mut payload = vec![];

    for c in candles {
        payload.push(get_row(
            c.timestamp,
            c.open,
            c.high,
            c.low,
            c.close,
            c.volume,
        ));
        row_count += 1;
        if row_count >= batch_size {
            insert_into(get_table()).values(&payload).execute(&conn)?;
            println!("Wrote {} candles to DB", row_count);
            payload.clear();
            row_count = 0;
        }
    }
    // There is still some rows remaining
    if row_count > 0 {
        insert_into(get_table()).values(&payload).execute(&conn)?;
        println!("Wrote {} candles to DB", row_count);
    }

    Ok(())
}

pub fn get_all_candles(table_name: &str) -> Result<Vec<Candle>> {
    let conn = get_db_connection()?;
    let supported_table = SupportedTable::from_table_name(table_name)?;

    let results = match supported_table {
        SupportedTable::UsdtBtc(table) => table.load::<Candle>(&conn),
    }?;

    Ok(results)
}
