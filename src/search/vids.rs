use anyhow::{anyhow, Result};
use askama::Template;
use axum::{debug_handler, response::IntoResponse, Form};
use schizosearch::{fetch, HtmlTemplate};
use serde_json::Value;

use super::SearchQuery;

#[derive(Debug)]
pub struct ResultVideo {
    pub title: String,
    pub link: String,
    pub thumbnail: String,
}

#[derive(Template)]
#[template(path = "vids.html")]
pub struct VideosPage {
    pub query: String,
    pub results: Vec<ResultVideo>,
}

#[debug_handler]
pub async fn vids_search(Form(query): Form<SearchQuery>) -> impl IntoResponse {
    let query = query.q;
    let results = indivious(&query).await.unwrap();
    let page = VideosPage { query, results };
    HtmlTemplate(page)
}

pub async fn indivious(query: &str) -> Result<Vec<ResultVideo>> {
    let json = fetch!(
        "https://invidious.protokolla.fi/api/v1/search?q={}&type=video",
        query
    );

    let data: Value = serde_json::from_str(&json)?;
    if let Some(array) = data.as_array() {
        let results = array
            .iter()
            .map(|v| ResultVideo {
                link: format!(
                    "https://invidious.protokolla.fi/watch?v={}",
                    v.get("videoId").unwrap().as_str().unwrap().to_owned()
                ),
                title: v.get("title").unwrap().as_str().unwrap().to_owned(),
                thumbnail: v["videoThumbnails"][5]
                    .get("url")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_owned(),
            })
            .collect();
        Ok(results)
    } else {
        Err(anyhow!("Failed to fetch indivious videos."))
    }
}
