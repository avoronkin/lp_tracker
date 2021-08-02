extern crate clap;
use chrono::prelude::*;
use clap::Clap;

#[derive(Clap, Debug)]
#[clap(name = "stock_tracker")]
struct Opts {

    #[clap(short, long)]
    symbol: String,

    #[clap(short, long)]
    from: NaiveDateTime,
}

#[derive(Debug)]
pub struct Config {
    pub symbol: String,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

impl Config {
    pub fn new() -> Self {
        let opts = Opts::parse();

        Self {
            symbol: opts.symbol,
            from: Utc.from_utc_datetime(&opts.from),
            to: Utc::now(),
        }
    }
}
