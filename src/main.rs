use colored::Colorize;

#[derive(serde_derive::Deserialize)]
struct Quote {
    content: String,
    author: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://api.quotable.io/random").await?;

    let quote: Quote = response.json().await?;

    println!("{} \n - {}", quote.content.green(), quote.author.cyan());

    Ok(())
}
