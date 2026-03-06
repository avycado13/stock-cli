use std::env;

use serde::Deserialize;
// use chrono::NaiveDate;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "Meta Data")]
    pub meta_data: MetaData,

    #[serde(rename = "Time Series (Daily)")]
    pub time_series_daily: BTreeMap<String, DailyBar>,
}

#[derive(Debug, Deserialize)]
pub struct MetaData {
    #[serde(rename = "1. Information")]
    pub information: String,

    #[serde(rename = "2. Symbol")]
    pub symbol: String,

    #[serde(rename = "3. Last Refreshed")]
    pub last_refreshed: String,

    #[serde(rename = "4. Output Size")]
    pub output_size: String,

    #[serde(rename = "5. Time Zone")]
    pub time_zone: String,
}

#[derive(Debug, Deserialize)]
pub struct DailyBar {
    #[serde(rename = "1. open")]
    pub open: String,

    #[serde(rename = "2. high")]
    pub high: String,

    #[serde(rename = "3. low")]
    pub low: String,

    #[serde(rename = "4. close")]
    pub close: String,

    #[serde(rename = "5. volume")]
    pub volume: String,
}

fn main() {
     let args: Vec<String> = env::args().collect();
 let ticker = args.get(1).map(String::as_str).unwrap_or("IBM");
let days: usize = args
    .get(2)
    .and_then(|d| d.parse().ok())
    .unwrap_or(5);   let client = reqwest::blocking::Client::new();
       let raw_quote: ApiResponse = client
        .get(format!("https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol={}&apikey=NVRP9BI9YDIIPZPY",ticker))
        .send()
        .expect("Failed to fetch quote")
        .json()
        .expect("Failed to parse quote");

    println!("Past {} day closing for {:#?}", days,raw_quote.meta_data.symbol);

    for (date, bar) in raw_quote.time_series_daily.iter().rev().take(days){
        println!("{} -> close {}", date, bar.close);
    }}
