use clap::Parser;
use colored::Colorize;
use futures::future::join_all;
use reqwest::Error;
use ruquotes::{base_url, quote_from, API_URL_ENV};
use std::process::exit;

#[derive(Parser)]
#[clap(
    author = "Eliaz Bobadilla",
    version = "0.2.0",
    about = "Print random quotes."
)]
struct Args {
    #[clap(short, long, default_value_t = 1)]
    times: usize,

    /// Base URL of a Quotable server (default: $QUOTABLE_API_URL, then https://api.quotable.io)
    #[clap(long)]
    api_url: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    let api_url = args.api_url.as_deref();

    let mut futures = Vec::new();

    for _ in 0..args.times {
        futures.push(async {
            let quote = match quote_from(api_url).await {
                Ok(quote) => quote,
                Err(e) => {
                    eprintln!("Could not get a quote from {}.", base_url(api_url));
                    eprintln!("Pass --api-url or set {API_URL_ENV} to point at a running Quotable server.");
                    eprintln!("{}", e);
                    exit(1);
                }
            };

            println!("{} \n - {}", quote.content.green(), quote.author.cyan());
        });
    }

    join_all(futures).await;

    Ok(())
}
