use clap::Parser;
use colored::Colorize;
use futures::future::join_all;
use reqwest::Error;
use ruquotes::quote;
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
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    let mut futures = Vec::new();

    for _ in 0..args.times {
        futures.push(async {
            let quote = match quote().await {
                Ok(quote) => quote,
                Err(e) => {
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
