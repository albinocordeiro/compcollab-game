use crate::data::model::Candle;
use color_eyre::Result;
use std::sync::mpsc::Sender;
use crate::data::dbapi::get_all_candles;

pub struct BackTestRunner {
    strategies: Vec<Sender<Candle>>,
    currency_pair: String,  
}

impl BackTestRunner {
    pub fn subscribe(&mut self, strategy_pipe: Sender<Candle>) -> Result<()> {
        self.strategies.push(strategy_pipe);
        Ok(())
    }

    pub fn start(&self) -> Result<()> {
        let candles = get_all_candles(&self.currency_pair).unwrap();
        for candle in candles {
            for strategy_pipe in &self.strategies {
                strategy_pipe.send(candle.clone())?;
            }
        }
        Ok(())
    }
}