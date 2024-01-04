use anyhow::{anyhow, Result};
use schizosearch::fetch;
use serde_json::Value;

#[derive(Debug)]
pub struct ResultVideo {
    pub title: String,
    pub link: String,
    pub thumbnail: String,
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
