use anyhow::Result;
use askama::Template;
use axum::{debug_handler, response::IntoResponse, Form};
use schizosearch::{fetch, HtmlTemplate};
use serde_json::Value;

use super::SearchQuery;

#[derive(Debug)]
pub struct ResultImage {
    pub alt: String,
    pub image_url: String,
    pub link: String,
}

#[derive(Template)]
#[template(path = "img.html")]
pub struct ImagesPage {
    pub query: String,
    pub results: Vec<ResultImage>,
}

#[debug_handler]
pub async fn img_search(Form(query): Form<SearchQuery>) -> impl IntoResponse {
    let query = query.q;
    let results = qwant(&query).await.unwrap();
    let page = ImagesPage { query, results };
    HtmlTemplate(page)
}
pub async fn qwant(query: &str) -> Result<Vec<ResultImage>> {
    let json = fetch!("https://api.qwant.com/v3/search/images?q={}&t=images&count=50&locale=en_us&offset=0&device=desktop&tgp=3&safesearch=1", query);

    let data: Value = serde_json::from_str(&json)?;

    // FIXME: holy fucking shit
    let results: Vec<ResultImage> = data["data"]["result"]["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| ResultImage {
            alt: f.get("title").unwrap().as_str().unwrap().to_owned(),
            image_url: f.get("media_preview").unwrap().as_str().unwrap().to_owned(),
            link: f
                .get("media_fullsize")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned(),
        })
        .collect();

    Ok(results)
}
