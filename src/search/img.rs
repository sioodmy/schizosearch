use anyhow::{anyhow, Result};
use schizosearch::fetch;
use serde_json::Value;

use super::ResultImage;

pub async fn qwant(query: &str) -> Result<Vec<ResultImage>> {
    let json = fetch!("https://api.qwant.com/v3/search/images?q={}&t=images&count=50&locale=en_us&offset=0&device=desktop&tgp=3&safesearch=1", query);

    let data: Value = serde_json::from_str(&json)?;

    // FIXME: holy fucking shit
    if let Some(results) = data["data"]["result"]["items"].as_array() {
        let results = results
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
    } else {
        Err(anyhow!("Failed to parse qwant data"))
    }
}
