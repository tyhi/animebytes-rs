//! # animebytes-rs
//!
//! `animebytes-rs` is a crate that provides access to animebytes-tv tracker
//! api. You need to provide your torrent password to access most of the
//! endpoints. Prometheus endpoints are not provided however in the future there
//! is a chance that they could be added if requested.

#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::perf)]
#![allow(clippy::module_name_repetitions, clippy::no_effect_underscore_binding)]
use serde::de::DeserializeOwned;

pub mod errors;
pub mod models;

pub struct Client {
    torrent_pass: String,
    username: String,
    http_client: reqwest::Client,
}

impl Client {
    /// Create a new `Client`, providing a torrent password, and username.
    /// Current checking if credentials are valid is not done on client
    /// creation.
    /// # Errors
    /// This method fails if the http client backend could not be created.
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
    #[tokio::test]
    async fn test_anime_search() {
        let client = crate::Client::new(&std::env::var("AB_KEY").unwrap(), &std::env::var("AB_USER").unwrap()).unwrap();

        let dto = client.search_anime("sword art online").await.unwrap();

        assert!(dto.groups.iter().any(|x| x.id == 12053));
    }

    #[tokio::test]
    async fn test_status() {
        let client = crate::Client::new(&std::env::var("AB_KEY").unwrap(), &std::env::var("AB_USER").unwrap()).unwrap();

        let dto = client.status().await.unwrap();

        assert!(dto.success);
    }

    #[tokio::test]
    async fn test_stats() {
        let client = crate::Client::new(&std::env::var("AB_KEY").unwrap(), &std::env::var("AB_USER").unwrap()).unwrap();

        let dto = client.stats().await.unwrap();

        assert!(dto.success);
    }
}
