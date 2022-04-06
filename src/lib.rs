use reqwest::{get, Error};
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Quote {
    #[serde(rename(deserialize = "_id"))]
    pub id: String,
    pub tags: Vec<String>,
    pub content: String,
    pub author: String,
    #[serde(rename(deserialize = "authorSlug"))]
    pub author_slug: String,
    pub length: u64,
    #[serde(rename(deserialize = "dateAdded"))]
    pub date_added: String,
    #[serde(rename(deserialize = "dateModified"))]
    pub date_modified: String,
}

pub const API_URL: &str = "https://api.quotable.io/random";

pub async fn quote() -> Result<Quote, Error> {
    let response = get(API_URL).await?;

    let quote: Quote = response.json().await?;

    Ok(quote)
}

#[cfg(test)]
mod tests {
    use std::any::type_name;

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }

    #[tokio::test]
    async fn quote() {
        let data = super::quote().await.unwrap();

        assert_eq!("u64", type_of(data.length));
    }
}
