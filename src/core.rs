use color_eyre::Result;
use std::sync::mpsc::Receiver;
use crate::data::model::Candle;

pub trait Strategy {
    fn get_input_pipe(self) -> Result<Receiver<Candle>>;
}