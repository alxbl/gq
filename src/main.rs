use clap::{App, Arg, values_t};
use reqwest;
use serde_json::Value;
use std::fmt;
use tokio;
use tokio::task::JoinHandle;

#[derive(Debug)]
struct ParserError;

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not parse symbol value")
    }
}

impl std::error::Error for ParserError {}

pub struct Quote {
    pub date: String, // Date?
    pub symbol: String,
    pub value: f64
}

impl fmt::Display for Quote {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.symbol, self.value)
    }
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// FIXME: Change to a proper monetary decimal format?
async fn get_quote(symbol: String) -> Result<Quote> {
    let url = format!("https://query1.finance.yahoo.com/v8/finance/chart/{}?lang=en-US&region=US&interval=1d&range=1d", symbol);
    let c = reqwest::Client::new();
    let r: Value = c.get(&url).send().await?.json()
        .await?;

    let value = r["chart"]["result"][0]["meta"]["chartPreviousClose"]
        .as_f64()
        .ok_or(Box::new(ParserError))?;

    Ok(Quote { value, symbol, date: String::from("TODO") })
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("gq")
        .version("1.0")
        .about("Get symbol quotes from Yahoo Finance")
        .arg(
            Arg::with_name("SYMBOL")
                .help("Symbol to retrieve")
                .required(true)
                .multiple(true)
                .index(1),
        )
        .get_matches();

    let symbols: Vec<String> = values_t!(matches, "SYMBOL", String).unwrap_or_else(|e| e.exit());

    let handles: Vec<JoinHandle<_>> = symbols.iter()
        .map(|s| {
            let symbol = s.clone();
            tokio::spawn(async move {
                if let Ok(q) = get_quote(symbol).await {
                    println!("{}", q)
                }
            })
        }).collect();

    for h in handles.into_iter() {
        let _ = h.await?;
    }

    Ok(())
}
