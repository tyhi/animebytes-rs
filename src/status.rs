use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{errors::Error, Client};

#[async_trait]
pub trait Status {
    async fn status(&self) -> Result<StatusDTO, Error>;
}

#[async_trait]
impl Status for Client {
    async fn status(&self) -> Result<StatusDTO, Error> { self.get("https://status.animebytes.tv/api/status").await }
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
