use std::collections::HashMap;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{errors, errors::Error, Client};

#[async_trait]
pub trait Search {
    async fn search_anime(&self, search: &str) -> Result<SearchResult, Error>;
    async fn search_music(&self, search: &str) -> Result<SearchResult, Error>;
}

#[async_trait]
impl Search for Client {
    async fn search_anime(&self, search: &str) -> Result<SearchResult, errors::Error> {
        self.get(&format!(
            "https://animebytes.tv/scrape.php?torrent_pass={}&username={}&type=anime&searchstr={}",
            self.torrent_pass, self.username, search
        ))
        .await
    }

    async fn search_music(&self, search: &str) -> Result<SearchResult, Error> { todo!() }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    #[serde(rename = "Matches")]
    pub matches: i64,
    #[serde(rename = "Limit")]
    pub limit: i64,
    #[serde(rename = "Results")]
    pub results: String,
    #[serde(rename = "Groups")]
    pub groups: Vec<Group>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "CategoryName")]
    pub category_name: CategoryName,
    #[serde(rename = "FullName")]
    pub full_name: String,
    #[serde(rename = "GroupName")]
    pub group_name: String,
    #[serde(rename = "SeriesID")]
    pub series_id: String,
    #[serde(rename = "SeriesName")]
    pub series_name: String,
    #[serde(rename = "Artists")]
    pub artists: Option<serde_json::Value>,
    #[serde(rename = "Year")]
    pub year: String,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "Synonymns")]
    pub synonymns: Synonymns,
    #[serde(rename = "Snatched")]
    pub snatched: i64,
    #[serde(rename = "Comments")]
    pub comments: i64,
    #[serde(rename = "Links")]
    pub links: LinksUnion,
    #[serde(rename = "Votes")]
    pub votes: i64,
    #[serde(rename = "AvgVote")]
    pub avg_vote: f64,
    #[serde(rename = "Associations")]
    pub associations: Option<serde_json::Value>,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "DescriptionHTML")]
    pub description_html: String,
    #[serde(rename = "EpCount")]
    pub ep_count: i64,
    #[serde(rename = "StudioList")]
    pub studio_list: Option<String>,
    #[serde(rename = "PastWeek")]
    pub past_week: i64,
    #[serde(rename = "Incomplete")]
    pub incomplete: bool,
    #[serde(rename = "Ongoing")]
    pub ongoing: bool,
    #[serde(rename = "Tags")]
    pub tags: Vec<String>,
    #[serde(rename = "Torrents")]
    pub torrents: Vec<Torrent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinksClass {
    #[serde(rename = "AniDB")]
    pub ani_db: Option<String>,
    #[serde(rename = "ANN")]
    pub ann: Option<String>,
    #[serde(rename = "Wikipedia")]
    pub wikipedia: Option<String>,
    #[serde(rename = "MAL")]
    pub mal: Option<String>,
    #[serde(rename = "Manga-Updates")]
    pub manga_updates: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Torrent {
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "EditionData")]
    pub edition_data: EditionData,
    #[serde(rename = "RawDownMultiplier")]
    pub raw_down_multiplier: f64,
    #[serde(rename = "RawUpMultiplier")]
    pub raw_up_multiplier: f64,
    #[serde(rename = "Link")]
    pub link: String,
    #[serde(rename = "Property")]
    pub property: String,
    #[serde(rename = "Snatched")]
    pub snatched: i64,
    #[serde(rename = "Seeders")]
    pub seeders: i64,
    #[serde(rename = "Leechers")]
    pub leechers: i64,
    #[serde(rename = "Size")]
    pub size: i64,
    #[serde(rename = "FileCount")]
    pub file_count: i64,
    #[serde(rename = "UploadTime")]
    pub upload_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditionData {
    #[serde(rename = "EditionTitle")]
    pub edition_title: String,
    #[serde(rename = "ReleaseDate")]
    pub release_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LinksUnion {
    AnythingArray(Vec<Option<serde_json::Value>>),
    LinksClass(LinksClass),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Synonymns {
    StringArray(Vec<String>),
    StringMap(HashMap<String, String>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CategoryName {
    Anime,
    Artbook,
    Game,
    #[serde(rename = "Light Novel")]
    LightNovel,
    Manga,
}
