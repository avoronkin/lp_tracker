use anyhow::Result;
use chrono::prelude::*;
use serde::Serialize;
use std::io;
use yahoo_finance_api as yahoo;

mod config;
mod metrics;

fn main() {
    let config = config::Config::new();

    if let Err(err) = run(config) {
        eprintln!("{:?}!", err);
        std::process::exit(1);
    }
}

fn run(config: config::Config) -> Result<()> {
    let quotes = get_quotes(&config)?;

    if let Some(record) = get_record(quotes, &config.symbol) {
        let mut wtr = csv::Writer::from_writer(io::stdout());
        wtr.serialize(record)?;
        wtr.flush()?;
    }

    Ok(())
}

fn get_quotes(config: &config::Config) -> Result<Vec<yahoo_finance_api::Quote>> {
    let provider = yahoo::YahooConnector::new();
    let response = provider.get_quote_history(&config.symbol, config.from, config.to)?;
    let quotes = response.quotes()?;

    Ok(quotes)
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    date: String,
    symbol: String,
    close: String,
    change: String,
    minimum: String,
    maximum: String,
    sma30: String,
}

fn get_record(quotes: Vec<yahoo_finance_api::Quote>, symbol: &str) -> Option<Record> {
    let close_prices: Vec<f64> = quotes.iter().map(|q| q.close).collect();

    let record = Record {
        date: Utc
            .timestamp(quotes.last()?.timestamp as i64, 0)
            .to_rfc3339(),
        symbol: symbol.to_string(),
        close: format!("${:.2}", quotes.last()?.close),
        change: format!("{:.2}%", metrics::price_diff(&close_prices)?.0),
        minimum: format!("${:.2}", metrics::min(&close_prices)?),
        maximum: format!("${:.2}", metrics::max(&close_prices)?),
        sma30: format!("${:.2}", metrics::n_window_sma(30, &close_prices)?.last()?),
    };

    Some(record)
}
