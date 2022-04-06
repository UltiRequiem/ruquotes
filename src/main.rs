use colored::Colorize;
use ruquotes::quote;
use std::process::exit;

#[tokio::main]
async fn main() {
    let custom_quote = match quote().await {
        Ok(quote) => quote,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    println!(
        "{} \n - {}",
        custom_quote.content.green(),
        custom_quote.author.cyan()
    );
}
