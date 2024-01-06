use anyhow::Result;
use schizosearch::fetch;
use scraper::{Html, Selector};

use super::ResultsMap;

pub async fn duckduckgo(query: &str, map: &ResultsMap) -> Result<()> {
    let html = fetch!("https://html.duckduckgo.com/html/?q={}&kd=-1", query);
    let fragment = Html::parse_document(&html);
    let selector = Selector::parse("div.web-result").unwrap();
    for result in fragment.select(&selector) {
        // TODO async
        let selector = Selector::parse("a.result__a").unwrap();
        let Some(element) = result.select(&selector).next() else {
            continue;
        };
        let Some(link) = element.value().attr("href") else {
            continue;
        };

        let desc_selector = Selector::parse("a.result__snippet").unwrap();
        let Some(desc_element) = result.select(&desc_selector).next() else {
            continue;
        };

        let title = element.text().collect::<Vec<_>>().join("");
        let description = desc_element.text().collect::<Vec<_>>().join("");

        let _ = map.insert(
            link.to_owned(),
            super::ResultGeneralData { title, description },
        );
    }

    Ok(())
}
