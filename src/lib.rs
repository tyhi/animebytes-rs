#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::perf)]
#![allow(clippy::module_name_repetitions)]
use serde::de::DeserializeOwned;

mod errors;
mod search;
mod stats;
mod status;

pub struct Client {
    torrent_pass: String,
    username: String,
    http_client: reqwest::Client,
}

impl Client {
    /// # Errors
    pub fn new(torrent_pass: &str, username: &str) -> Result<Self, errors::Error> {
        Ok(Self {
            torrent_pass: torrent_pass.into(),
            username: username.into(),
            http_client: reqwest::ClientBuilder::new().build()?,
        })
    }

    async fn get<T: DeserializeOwned>(&self, params: &str) -> Result<T, errors::Error> {
        Ok(self.http_client.get(params).send().await?.json().await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::{search::Search, stats::Stats};

    #[tokio::test]
    async fn test_anime_search() {
        let client = crate::Client::new("EFtn1jRe8KdY174GBkGRM0RGHBh23gvI", "Seoyne").unwrap();

        let dto = client.search_anime("sword art online").await.unwrap();

        assert!(dto.groups.iter().any(|x| x.id == 12053));
    }

    #[tokio::test]
    async fn test_status() {
        let client = crate::Client::new(&std::env::var("AB_KEY").unwrap(), "Seoyne").unwrap();

        let dto = client.stats().await.unwrap();

        assert!(dto.success);
    }
}
