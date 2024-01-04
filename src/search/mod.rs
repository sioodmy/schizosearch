use askama_enum::EnumTemplate;
use axum::{debug_handler, response::IntoResponse, Form};
use schizosearch::HtmlTemplate;
use serde::Deserialize;
use tokio::try_join;

pub use img::img_search;
pub use vids::vids_search;

mod brave;
mod duckduckgo;
pub mod img;
mod tests;
pub mod vids;

#[derive(Deserialize, Debug)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Debug)]
pub struct ResultHtml {
    pub title: String,
    pub link: String,
    pub description: String,
}

#[derive(EnumTemplate)]
pub enum ResultPage {
    #[template(path = "results.html")]
    General {
        query: String,
        results: Vec<ResultHtml>,
    },
    #[template(path = "img.html")]
    Images {
        query: String,
        results: Vec<img::ResultImage>,
    },
    #[template(path = "vids.html")]
    Videos {
        query: String,
        results: Vec<vids::ResultVideo>,
    },
}

#[debug_handler]
pub async fn search(Form(query): Form<SearchQuery>) -> impl IntoResponse {
    let mut results = Vec::new();
    let query = query.q;
    if let Ok((brave, duckduckgo)) = try_join!(brave::brave(&query), duckduckgo::duckduckgo(&query))
    {
        results.extend(brave);
        results.extend(duckduckgo);
    } else {
        panic!("TODO: find a better way of handling this");
    }
    let page = ResultPage::General { query, results };
    HtmlTemplate(page)
}
