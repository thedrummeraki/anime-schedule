use std::fmt::Display;

use clap_derive::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, ValueEnum)]
pub enum Season {
    Winter,
    Spring,
    Summer,
    Fall,
}

impl From<Season> for String {
    fn from(value: Season) -> Self {
        match value {
            Season::Fall => "FALL".into(),
            Season::Winter => "WINTER".into(),
            Season::Spring => "SPRING".into(),
            Season::Summer => "SUMMER".into(),
        }
    }
}

impl Serialize for Season {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: String = self.clone().into();
        serializer.serialize_str(&value)
    }
}

#[derive(Debug, Clone, ValueEnum, Serialize)]
pub enum ShowStatus {
    Airing,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GraphqlResponse<T> {
    pub data: GraphqlPage<T>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GraphqlPage<T> {
    #[serde(rename = "Page")]
    pub page: Option<GraphqlPageContents<T>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GraphqlPageContents<T> {
    #[serde(rename = "pageInfo")]
    pub page_info: GraphqlPageInfo,
    pub media: Vec<T>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GraphqlPageInfo {
    pub total: i32,
    #[serde(rename = "hasNextPage")]
    pub hax_next_page: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Show {
    pub id: i64,
    pub title: Title,
    pub episodes: Option<i32>,
    pub duration: Option<f32>,
    #[serde(rename = "nextAiringEpisode")]
    pub next_airing_episode: Option<NextAiringEpisode>,
}

impl Display for Show {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = &self.title;
        f.write_str(&format!(
            "{}",
            title
                .english
                .as_ref()
                .or(title.native.as_ref())
                .unwrap_or(&title.user_preferred)
        ))
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Title {
    pub english: Option<String>,
    pub native: Option<String>,
    pub romaji: Option<String>,
    #[serde(rename = "userPreferred")]
    pub user_preferred: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct NextAiringEpisode {
    #[serde(rename = "airingAt")]
    pub airing_at: i64,
    pub episode: i32,
}

#[derive(Serialize, Clone, Debug)]
pub struct FindMultipleShowsVariables {
    pub page: i32,
    pub sort: String,
    pub status: String,
    #[serde(rename = "type")]
    pub show_type: String,
    pub season: Option<Season>,
    #[serde(rename = "seasonYear")]
    pub season_year: Option<i32>,
}

#[derive(Serialize, Clone, Debug)]
pub struct GraphqlRequestBody<T>
where
    T: Serialize,
{
    pub query: String,
    pub variables: Option<T>,
}
