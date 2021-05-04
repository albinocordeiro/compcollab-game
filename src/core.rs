use crate::data::model::Candle;
use color_eyre::Result;
use std::sync::mpsc::Receiver;

pub trait Strategy {
    fn get_input_pipe(self) -> Result<Receiver<Candle>>;
}
