use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{errors::Error, Client};

#[async_trait]
pub trait Stats {
    async fn stats(&self) -> Result<StatsDTO, Error>;
}

#[async_trait]
impl Stats for Client {
    async fn stats(&self) -> Result<StatsDTO, Error> {
        self.get(&format!("https://animebytes.tv/api/stats/{}", self.torrent_pass))
            .await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsDTO {
    pub success: bool,
    pub git: String,
    pub api: Api,
    pub freeleech: Freeleech,
    pub stats: StatsClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Api {
    pub version: String,
    pub compat: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Freeleech {
    pub sitewide: i64,
    pub personal: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsClass {
    pub site: Site,
    pub personal: Personal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Personal {
    pub yen: Yen,
    pub hnrs: Hnrs,
    pub upload: Load,
    pub download: Load,
    pub torrents: PersonalTorrents,
    pub invited: i64,
    pub forums: Forums,
    pub pcomments: i64,
    pub class: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Load {
    pub raw: i64,
    pub accountable: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Forums {
    pub posts: i64,
    pub topics: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hnrs {
    pub potential: i64,
    pub active: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonalTorrents {
    pub seeding: i64,
    pub leeching: i64,
    pub snatched: i64,
    pub uploaded: i64,
    pub pruned: i64,
    pub ssize: i64,
    pub sttime: i64,
    pub satime: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Yen {
    pub day: i64,
    pub hour: i64,
    pub now: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Site {
    pub torrents: SiteTorrents,
    pub requests: Requests,
    pub users: Users,
    pub peers_uniq: Peers,
    pub peers: Peers,
    pub classes: Classes,
    pub forums: Forums,
    pub donations: Donations,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Classes {
    #[serde(rename = "Aka-chan")]
    pub aka_chan: i64,
    #[serde(rename = "User")]
    pub user: i64,
    #[serde(rename = "Power User")]
    pub power_user: i64,
    #[serde(rename = "Elite")]
    pub elite: i64,
    #[serde(rename = "Torrent Master")]
    pub torrent_master: i64,
    #[serde(rename = "Legend")]
    pub legend: i64,
    #[serde(rename = "VIP")]
    pub vip: i64,
    #[serde(rename = "Sensei")]
    pub sensei: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Donations {
    pub currency: String,
    pub goal: i64,
    pub collected: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Peers {
    pub seeders: i64,
    pub leechers: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Requests {
    pub filled: i64,
    pub all: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteTorrents {
    pub active: i64,
    pub all: i64,
    pub snatches: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    pub enabled: i64,
    pub now: i64,
    pub day: i64,
    pub week: i64,
    pub month: i64,
    pub irc: i64,
}
