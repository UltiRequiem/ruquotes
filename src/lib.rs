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

/// The public Quotable host, used when nothing else is configured.
pub const DEFAULT_API_URL: &str = "https://api.quotable.io";

/// Environment variable that overrides [`DEFAULT_API_URL`].
pub const API_URL_ENV: &str = "QUOTABLE_API_URL";

/// The full URL of the random quote endpoint on the public host.
#[deprecated(note = "the base URL is configurable now, use `quote_from` or `QUOTABLE_API_URL`")]
pub const API_URL: &str = "https://api.quotable.io/random";

/// Pick the server: the explicit value, then the environment value, then the
/// public default. Empty values are ignored and trailing slashes removed.
pub fn resolve_base_url(explicit: Option<&str>, env: Option<&str>) -> String {
    let url = explicit
        .filter(|url| !url.is_empty())
        .or(env.filter(|url| !url.is_empty()))
        .unwrap_or(DEFAULT_API_URL);

    url.trim_end_matches('/').to_owned()
}

/// The server to talk to, reading `QUOTABLE_API_URL` from the environment.
pub fn base_url(explicit: Option<&str>) -> String {
    resolve_base_url(explicit, std::env::var(API_URL_ENV).ok().as_deref())
}

/// A random quote from the configured server (`QUOTABLE_API_URL`, or the
/// public default when it is not set).
pub async fn quote() -> Result<Quote, Error> {
    quote_from(None).await
}

/// A random quote from the Quotable server at `base_url`, for example
/// `https://quotes.example.com`. `None` falls back to `QUOTABLE_API_URL` and
/// then to [`DEFAULT_API_URL`].
pub async fn quote_from(base_url_override: Option<&str>) -> Result<Quote, Error> {
    let url = format!("{}/random", base_url(base_url_override));

    let response = get(url).await?.error_for_status()?;

    response.json().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    #[test]
    fn explicit_value_wins_over_the_environment() {
        assert_eq!(
            resolve_base_url(Some("https://a.example.com"), Some("https://b.example.com")),
            "https://a.example.com"
        );
    }

    #[test]
    fn environment_wins_over_the_default() {
        assert_eq!(
            resolve_base_url(None, Some("https://env.example.com/")),
            "https://env.example.com"
        );
    }

    #[test]
    fn falls_back_to_the_public_host_and_ignores_empty_values() {
        assert_eq!(resolve_base_url(None, None), DEFAULT_API_URL);
        assert_eq!(resolve_base_url(Some(""), Some("")), DEFAULT_API_URL);
    }

    #[test]
    fn trailing_slashes_are_removed() {
        assert_eq!(
            resolve_base_url(Some("https://h.example.com///"), None),
            "https://h.example.com"
        );
    }

    /// A one-request HTTP server: answers with `status` and `body`, returns the request it saw.
    async fn serve_once(
        status: &'static str,
        body: &'static str,
    ) -> (String, tokio::task::JoinHandle<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();

        let handle = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = [0u8; 4096];
            let read = socket.read(&mut buffer).await.unwrap();

            let response = format!(
                "HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
                body.len()
            );
            socket.write_all(response.as_bytes()).await.unwrap();

            String::from_utf8_lossy(&buffer[..read]).into_owned()
        });

        (format!("http://{address}"), handle)
    }

    const QUOTE: &str = r#"{"_id":"1","tags":[],"content":"Be yourself.","author":"Someone","authorSlug":"someone","length":12,"dateAdded":"a","dateModified":"b"}"#;

    #[tokio::test]
    async fn quote_from_uses_the_given_server() {
        let (url, server) = serve_once("200 OK", QUOTE).await;

        let quote = quote_from(Some(&format!("{url}/api/"))).await.unwrap();

        assert_eq!(quote.content, "Be yourself.");
        assert_eq!(quote.author, "Someone");
        assert_eq!(quote.length, 12);
        assert!(server.await.unwrap().starts_with("GET /api/random "));
    }

    #[tokio::test]
    async fn a_failing_server_is_an_error() {
        let (url, _server) = serve_once("503 Service Unavailable", "{}").await;

        assert!(quote_from(Some(&url)).await.is_err());
    }
}
