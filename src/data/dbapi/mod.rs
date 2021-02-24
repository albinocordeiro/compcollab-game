use crate::data::model::NewCandle;
use color_eyre::{eyre::eyre, Result};

pub fn insert_candle(table_name: &str, candle: &NewCandle) -> Result<()> {
    Err(eyre!("Not implemented yet"))
}