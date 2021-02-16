extern crate compcollab_game;
extern crate clap;

use std::env;
use clap::{App, Arg, ArgMatches};
use compcollab_game::data::sources::poloniex::{CandleRequestArgs, CandleData, datetime_string_to_timestamp, download_candles};
use csv::Writer as CsvWriter;


fn get_output_file_name(args: &CandleRequestArgs) -> String {
    let mut res = String::from("");
    res.push_str(&args.pair);
    res.push_str("_");
    res.push_str(&args.timeframe.to_string());
    res.push_str("_");
    res.push_str(&args.start_utc.to_string());
    res.push_str("_");
    res.push_str(&args.end_utc.to_string());
    res.push_str(".csv");
    res
}

fn write_candles_to_file(csv_file_name: &str, candles: &[CandleData]){
    let mut csvwriter = match CsvWriter::from_path(csv_file_name) {
        Ok(wr) => wr,
        Err(e) => panic!("Could not create CSV writer: {}", e)
    };

    for candle in candles {
        match csvwriter.serialize(&candle) {
            Err(e) => panic!("Could not serialize record: {}", e),
            _ => {}
        };
    }
    match csvwriter.flush(){
        Ok(_r) => println!("Csv writing done"),
        Err(e) => panic!("Error writing to csv file: {}", e)
    };
}
fn main() {    
    let app = App::new("Candly: candle data downloader")
                    .version("1.0")
                    .author("Albino Cordeiro <albino@sportlogiq.com>")
                    .about("Download and save candle data from Poloniex")
                    .template("{bin} ({version}) - {usage}")
                    .arg(Arg::with_name("pair")
                        .short("p")
                        .long("pair")
                        .value_name("CURRENCY-PAIR")
                        .help("Currency pair name")
                        .takes_value(true))
                    .arg(Arg::with_name("timeframe")
                        .short("t")
                        .help("Candle time frame. Must be more than valid period: 300, 900, 1800, 7200, 14400, 86400")
                        .long("timeframe")
                        .value_name("seconds")
                        .takes_value(true))
                    .arg(Arg::with_name("start")
                        .short("s")
                        .long("start")
                        .value_name("%Y-%m-%d %H:%M:%S")
                        .help("Start time")
                        .takes_value(true))
                    .arg(Arg::with_name("end")
                        .short("e")
                        .long("end")
                        .value_name("%Y-%m-%d %H:%M:%S")
                        .help("End time")
                        .takes_value(true))
                        .help_message("Print this help message.")
                        .version_message("Show version information.");
    
    let matches: ArgMatches = app.get_matches_from(env::args_os());
    let mut options: CandleRequestArgs = CandleRequestArgs::default();
    options.pair = matches
                    .value_of("pair")
                    .unwrap_or(&options.pair)
                    .to_string();
    let timeframe_string = matches
                            .value_of("timeframe")
                            .unwrap_or("300")
                            .to_string();
    options.timeframe = timeframe_string.parse().expect("Time frame provided can't be parsed to u64");
    options.start_utc = datetime_string_to_timestamp(matches
                                                    .value_of("start")
                                                    .unwrap_or("2020-1-1 12:00:00")
                                                    .to_string());
    options.end_utc = datetime_string_to_timestamp(matches
                                                    .value_of("end")
                                                    .unwrap_or("2021-2-1 12:00:00")
                                                    .to_string());
    
    eprintln!("Parsed arguments: {:?}", &options);
    let csv_file_name = get_output_file_name(&options);
    eprintln!("Output file name: {}", &csv_file_name);
    
    let candles = download_candles(&options);
    
    write_candles_to_file(&csv_file_name, &candles);
}
