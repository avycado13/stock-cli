use std::env;

use serde::Deserialize;
use std::collections::BTreeMap;
use chrono::{Duration, NaiveDate};
use textplots::{Chart, LabelFormat, Shape, TickDisplay, Plot, LabelBuilder, TickDisplayBuilder};

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "Meta Data")]
    pub meta_data: MetaData,

    #[serde(rename = "Time Series (Daily)")]
    pub time_series_daily: BTreeMap<NaiveDate, DailyBar>,
}

#[derive(Debug, Deserialize)]
pub struct MetaData {
    #[serde(rename = "1. Information")]
    pub information: String,

    #[serde(rename = "2. Symbol")]
    pub symbol: String,

    #[serde(rename = "3. Last Refreshed")]
    pub last_refreshed: NaiveDate,

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
    .unwrap_or(5);
let client = reqwest::blocking::Client::new();
       let raw_quote: ApiResponse = client
        .get(format!("https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol={}&apikey=NVRP9BI9YDIIPZPY",ticker))
        .send()
        .expect("Failed to fetch quote")
        .json()
        .expect("Failed to parse quote");

    println!("Past {} day closing for {:#?}", days,raw_quote.meta_data.symbol);

    for (date, bar) in raw_quote.time_series_daily.iter().rev().take(days){
        println!("{} -> close {}", date, bar.close);
    }
    println!("\n");
    let mut closes: Vec<(NaiveDate, f32)> = raw_quote
    .time_series_daily
    .iter()
    .rev()
    .take(days)
    .map(|(date, bar)| (*date, bar.close.parse::<f32>().unwrap()))
    .collect();

// reverse so oldest → newest for plotting
closes.reverse();

let start = closes.first().unwrap().0;
let end = closes.last().unwrap().0;

let data: Vec<(f32, f32)> = closes
    .iter()
    .map(|(date, close)| ((date.signed_duration_since(start).num_days()) as f32, *close))
    .collect();

Chart::new_with_y_range(
    180,
    60,
    0.0,
    (end - start).num_days() as f32,
    data.iter().map(|p| p.1).fold(f32::MAX, f32::min) - 1.0,
    data.iter().map(|p| p.1).fold(f32::MIN, f32::max) + 1.0,
)
.lineplot(&Shape::Lines(&data))
.x_label_format(LabelFormat::Custom(Box::new(move |val| {
    format!("{}", start + Duration::days(val as i64))
})))
.y_label_format(LabelFormat::Value)
.y_tick_display(TickDisplay::Sparse)
.nice();
}
