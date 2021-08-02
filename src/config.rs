extern crate clap;
use chrono::prelude::*;
use clap::Clap;

#[derive(Clap, Debug)]
#[clap(name = "stock_tracker")]
struct Opts {

    #[clap(short, long)]
    symbols: String,

    #[clap(short, long)]
    from: NaiveDateTime,
}

#[derive(Debug)]
pub struct Config {
    pub symbols: Vec<String>,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

impl Config {
    pub fn new() -> Self {
        let opts = Opts::parse();

        Self {
            symbols: opts.symbols.split(",").map(String::from).collect(),
            from: Utc.from_utc_datetime(&opts.from),
            to: Utc::now(),
        }
    }
}
