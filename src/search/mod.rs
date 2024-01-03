use askama::Template;
use axum::{debug_handler, response::IntoResponse, Form};
use schizosearch::HtmlTemplate;
use serde::Deserialize;
use tokio::try_join;

mod brave;
mod duckduckgo;
pub mod img;

#[derive(Deserialize, Debug)]
pub struct SearchQuery {
    pub q: String,
}

struct ResultHtml {
    title: String,
    link: String,
    description: String,
}
#[derive(Template)]
#[template(path = "results.html")]
struct ResultsPage {
    query: String,
    results: Vec<ResultHtml>,
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
    let page = ResultsPage { query, results };
    HtmlTemplate(page)
}
