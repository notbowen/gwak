use async_trait::async_trait;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Challenge {
    /// The challenge category (i.e.: web, pwn, rev)
    pub category: String,

    /// A list of URLs to download the files from
    pub files: Vec<String>,

    /// The URL or command to connect to the server (optional)
    pub connection_info: Option<String>,
}

#[async_trait]
pub trait Adapter {
    /// A function that returns a list of `Challenge` objects
    async fn get_challenges(&self) -> Vec<Challenge>;
}
