use serde::{Deserialize, Serialize};

use crate::{errors::Error, Client};

impl Client {
    /// Returns basic information about the tracker, auth required.
    /// ```rust
    /// # async fn run() -> Result<(), animebytes_rs::errors::Error> {
    /// let client = animebytes_rs::Client::new("password", "username")?;
    ///
    /// let search_result = client.status().await?;
    ///
    /// println!("{:?}", search_result);
    /// Ok(())
    /// # }
    /// ```
    /// # Errors
    /// This method can fail if there is a general HTTP error.
    pub async fn status(&self) -> Result<StatusDTO, Error> { self.get("https://status.animebytes.tv/api/status").await }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusDTO {
    pub success: bool,
    pub status: StatusClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusClass {
    pub tracker: Tracker,
    pub site: Irc,
    pub irc: Irc,
    pub mei: Irc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Irc {
    pub status: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tracker {
    pub status: i64,
    pub details: Vec<Detail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Detail {
    pub status: i64,
    pub ip: String,
}
