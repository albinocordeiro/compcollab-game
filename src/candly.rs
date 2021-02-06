#![macro_use]
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate chrono;
extern crate csv;
extern crate reqwest;
extern crate clap;
extern crate phf;

use std::env;
use reqwest::blocking::Client;
use clap::{App, Arg, ArgMatches};
use chrono::NaiveDateTime;
use serde_derive::{Serialize,Deserialize};
use csv::Writer as CsvWriter ;

use phf::phf_map;

static UNITS_MAP: phf::Map<&'static str, u64> = phf_map! {
    "MIN" => 60,
    "HOUR" => 3600,
    "DAY" => 86400,
    "WEEK" => 604800,
    "MONTH" => 18144000,
};

#[derive(Debug)]
struct CandlyArgs {
    pair: String,
    timeframe: u64,
    start_utc: i64,
    end_utc: i64
}

impl CandlyArgs {
    fn get_output_file_name(&self) -> String {
        let mut res = String::from("");
        res.push_str(&self.pair);
        res.push_str("_");
        res.push_str(&self.timeframe.to_string());
        res.push_str("_");
        res.push_str(&self.start_utc.to_string());
        res.push_str("_");
        res.push_str(&self.end_utc.to_string());
        res.push_str(".csv");
        res
    }
}

impl Default for CandlyArgs {
    fn default() -> CandlyArgs {
        CandlyArgs {
            pair: String::from("USDT_BTC"),
            timeframe: 3600,
            start_utc: NaiveDateTime::parse_from_str("2020-6-1 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap().timestamp(),
            end_utc: NaiveDateTime::parse_from_str("2020-7-1 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap().timestamp(),
        }
    }
}

#[derive(Serialize,Deserialize)]
struct TickData {
    date: u64,
    high: f64,
    low: f64,
    open: f64,
    close: f64,
    volume: f64,
    quoteVolume: f64,
    weightedAverage: f64,
}

fn timeframe_string_to_seconds(tf: String) -> u64 {
    let split: Vec<&str> = tf.split("-").collect();
    let mult: u64 = u64::from_str_radix(split[0], 32).unwrap();
    let unit = UNITS_MAP[split[1]];
    mult*unit
}

fn datetime_string_to_timestamp(time_string: String) -> i64 {
    NaiveDateTime::parse_from_str(&time_string, "%Y-%m-%d %H:%M:%S").unwrap().timestamp()
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
    let mut options: CandlyArgs = CandlyArgs::default();
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
    eprintln!("Output file name: {}", options.get_output_file_name());

    let client = Client::new();
    
    let url: &str= &(format!("https://poloniex.com/public?command=returnChartData&currencyPair={0}&start={1}&end={2}&resolution=auto&period={3}",
                             options.pair, options.start_utc, options.end_utc, options.timeframe));
    let body = match client.get(url).send() {
        Ok(resp) => match resp.text() {
            Ok(text) => {
                if (text.contains("error")){
                    panic!(text)
                }
                text
            },
            Err(_ee) => panic!("Got nothing from poloniex")
        },
        Err(e) => panic!("Failed web requeest: {}", e)
    };

    let json_array: Vec<TickData> = match serde_json::from_str(&body) {
        Ok(parsed) => parsed,
        Err(e) => panic!("Json parsing error: {}", e)
    };
    
    println!("Records downloaded {}", json_array.len());
    let mut csvwriter = match CsvWriter::from_path(options.get_output_file_name()) {
        Ok(wr) => wr,
        Err(e) => panic!("Could not create CSV writer: {}", e)
    };

    for jobj in &json_array {
        match csvwriter.serialize(&jobj) {
            Err(e) => panic!("Could not serialize record: {}", e),
            _ => {}
        };
    }
    match csvwriter.flush(){
        Ok(_r) => println!("Csv writing done"),
        Err(e) => panic!("Error writing to csv file: {}", e)
    };
    
    println!("Done.........")
}
