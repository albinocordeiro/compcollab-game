use super::model::{NewCandle, Candle};
use color_eyre::{Result, eyre::eyre};

use diesel::{prelude::*};
use diesel::insert_into;
use std::env;

struct TableNamespace <T,Cdate,Copen,Chigh,Clow,Cclose,Cvol> {
    table: T,
    datecol: Cdate,
    opencol: Copen,
    highcol: Chigh,
    lowcol: Clow,
    closecol: Cclose,
    volumecol: Cvol,
}

pub fn get_db_connection() -> Result<PgConnection> {
    use dotenv::dotenv;
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;

    let conn = PgConnection::establish(&database_url)?;
    Ok(conn)
}

pub fn insert_candles(table_name: &str, candles: &[NewCandle]) -> Result<()> {
    
    let table_namespace = match table_name {

        "usdt_btc" => {
            use crate::schema::usdt_btc;
            TableNamespace {
                table: usdt_btc::table,
                datecol: usdt_btc::columns::date,
                opencol: usdt_btc::columns::open,
                highcol: usdt_btc::columns::high,
                lowcol: usdt_btc::columns::low,
                closecol: usdt_btc::columns::close,
                volumecol: usdt_btc::columns::volume,
            }
        },
        _ => return Err(eyre!("Table not supported: {}", table_name))
    };

    let conn = get_db_connection()?;
    
    let mut payload = vec![];
    let batch_size = 1000usize;
    let mut count = 0usize;
    let ns = &table_namespace;
    for c in candles {
        payload.push((ns.datecol.eq(c.date), ns.opencol.eq(c.open), ns.highcol.eq(c.high), ns.lowcol.eq(c.low), ns.closecol.eq(c.close), ns.volumecol.eq(c.volume)));
        count += 1;
        if count >= batch_size {
            insert_into(ns.table).values(&payload).execute(&conn)?;
            // println!("\n\n\n{:?}", debug_query::<Pg, _>(&query));
            println!("Wrote {} candles to DB", count);
            payload.clear();
            count = 0;
        }
    }
    if count > 0 {
        insert_into(ns.table).values(&payload).execute(&conn)?;
        println!("Wrote {} candles to DB", count);
    }

    Ok(())
}

pub fn get_candles(table_name: &str, start: chrono::NaiveDateTime, end: chrono::NaiveDateTime) -> Result<Vec<Candle>> {
    
    // check that earliest record is <= start
    // check last record >= end
    // build/execute query to get candles
    // return Vec of candles
}