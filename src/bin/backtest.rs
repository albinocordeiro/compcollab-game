extern crate compcollab_game;
extern crate clap;
extern crate color_eyre;

use std::env;
use clap::{App, Arg, ArgMatches};
use color_eyre::Result;
use compcollab_game::back_test::BackTestRunner;
use compcollab_game::strategies::TrendFollowing;


fn main() -> Result<()> {    
    let app = App::new("Candly: candle data downloader")
                    .version("0.1.0")
                    .author("Albino Cordeiro <albino@intuitionlogic.com>")
                    .about("Download and save candle data from Poloniex")
                    .template("{bin} ({version}) - {usage}")
                    .arg(Arg::with_name("pair")
                        .short("p")
                        .long("pair")
                        .value_name("CURRENCY-PAIR")
                        .help("Currency pair name")
                        .takes_value(true))
                        .help_message("Print this help message.")
                        .version_message("Show version information.");
    
    let matches: ArgMatches = app.get_matches_from(env::args_os());
    let mut currency_pair: &str = "USDT_BTC";
    currency_pair = matches
                    .value_of("pair")
                    .unwrap_or(&currency_pair);
   
    println!("Backtesting for the currency pair: {:?}", currency_pair);
    println!("Trend Following Strategy");


}
