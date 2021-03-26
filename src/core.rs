use color_eyre::Result;
pub trait Strategy {
    fn register_new_candle(&mut self) -> Result<()>;
}