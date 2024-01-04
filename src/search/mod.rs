use askama_enum::EnumTemplate;
use axum::{debug_handler, response::IntoResponse, Form};
use schizosearch::HtmlTemplate;
use serde::Deserialize;
use tokio::try_join;

use self::{img::qwant, vids::indivious};

mod brave;
mod duckduckgo;
pub mod img;
mod tests;
pub mod vids;

#[derive(Deserialize, Debug)]
pub struct Parameters {
    pub q: String,
    #[serde(default = "default_type")]
    pub t: String,
}

#[derive(Debug)]
pub struct ResultHtml {
    pub title: String,
    pub link: String,
    pub description: String,
}

#[derive(Debug)]
pub struct ResultImage {
    pub alt: String,
    pub image_url: String,
    pub link: String,
}

#[derive(Debug)]
pub struct ResultVideo {
    pub title: String,
    pub link: String,
    pub thumbnail: String,
}

#[derive(EnumTemplate)]
pub enum ResultPage {
    #[template(path = "results.html")]
    General {
        results: Vec<ResultHtml>,
        query: String,
    },
    #[template(path = "img.html")]
    Images {
        results: Vec<ResultImage>,
        query: String,
    },
    #[template(path = "vids.html")]
    Videos {
        results: Vec<ResultVideo>,
        query: String,
    },
}

#[debug_handler]
pub async fn search(Form(params): Form<Parameters>) -> impl IntoResponse {
    let mut results = Vec::new();
    let query = params.q;
    let page = match params.t.as_str() {
        "img" => ResultPage::Images {
            results: qwant(&query).await.unwrap(),
            query,
        },
        "vid" => ResultPage::Videos {
            results: indivious(&query).await.unwrap(),
            query,
        },

        "general" => {
            if let Ok((brave, duckduckgo)) =
                try_join!(brave::brave(&query), duckduckgo::duckduckgo(&query))
            {
                results.extend(brave);
                results.extend(duckduckgo);
            } else {
                panic!("TODO: find a better way of handling this");
            }
            ResultPage::General { query, results }
        }
        _ => todo!("Error page"),
    };
    HtmlTemplate(page)
}

fn default_type() -> String {
    "general".to_string()
}
