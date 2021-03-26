use crate::core::Strategy;
use color_eyre::Result;

pub struct BackTestRunner {
    strategies: Vec<Box<dyn Strategy>>,
    
}

impl BackTestRunner {
    pub fn subcribe<T: Strategy + Clone + Copy>(&mut self, new_strategy: &mut T) -> Result<()> {
        self.strategies.push(Box<T>::new(new_strategy));
        Ok(())
    }

    pub fn start_async() -> Result<()> {
        
    }
}