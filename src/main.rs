use colored::Colorize;

#[derive(serde_derive::Deserialize)]
struct Quote {
    content: String,
    author: String,
}

const API_URL: &str = "https://api.quotable.io/random";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let response = reqwest::get(API_URL).await?;

    let quote: Quote = response.json().await?;

    println!("{} \n - {}", quote.content.green(), quote.author.cyan());

    Ok(())
}
