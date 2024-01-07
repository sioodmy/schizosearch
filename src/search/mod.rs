use askama_enum::EnumTemplate;
use axum::{debug_handler, response::IntoResponse, Form};
use scc::HashMap;
use schizosearch::HtmlTemplate;
use serde::Deserialize;
use tokio::{join, sync::mpsc::channel};

use self::{img::qwant, libre::libre, special::SpecialResult, vids::indivious};

mod brave;
mod duckduckgo;
pub mod img;
mod libre;
mod special;
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

pub type ResultsMap = HashMap<String, ResultGeneralData>;
#[derive(Debug)]
pub struct ResultGeneralData {
    pub title: String,
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
        special: SpecialResult,
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
            let map: ResultsMap = HashMap::default();
            let (tx, mut rx) = channel::<SpecialResult>(1);
            let _ = join!(
                special::special(&query, tx),
                brave::brave(&query, &map),
                duckduckgo::duckduckgo(&query, &map)
            );
            let mut results = Vec::new();
            map.retain(|url, data| {
                results.push(ResultHtml {
                    title: data.title.clone(),
                    link: libre(url.clone()),
                    description: data.description.clone(),
                });
                false
            });
            let special = if let Some(special_recv) = rx.recv().await {
                special_recv
            } else {
                SpecialResult::Empty
            };
            ResultPage::General {
                special,
                query,
                results,
            }
        }
        _ => todo!("Error page"),
    };
    HtmlTemplate(page)
}

fn default_type() -> String {
    "general".to_string()
}
