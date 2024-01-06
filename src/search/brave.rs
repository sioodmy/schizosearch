use anyhow::Result;
use schizosearch::fetch;
use scraper::{Html, Selector};

use super::ResultsMap;

pub async fn brave(query: &str, map: &ResultsMap) -> Result<()> {
    let html = fetch!(
        "https://search.brave.com/search?q={}&nfpr=1&spellcheck=0",
        query
    );
    let fragment = Html::parse_document(&html);
    let selector = Selector::parse("div.snippet").unwrap();
    for result in fragment.select(&selector) {
        // TODO async
        let selector = Selector::parse("a.h").unwrap();
        let Some(element) = result.select(&selector).next() else {
            continue;
        };
        let Some(link) = element.value().attr("href") else {
            continue;
        };

        let title_selector = Selector::parse("div.heading-serpresult").unwrap();
        let desc_selector = Selector::parse("div.snippet-description").unwrap();
        let Some(title_element) = element.select(&title_selector).next() else {
            continue;
        };
        let Some(desc_element) = result.select(&desc_selector).next() else {
            continue;
        };

        let title = title_element.text().collect::<Vec<_>>().join("");
        let description = desc_element.text().collect::<Vec<_>>().join("");

        let _ = map.insert(
            link.to_owned(),
            super::ResultGeneralData { title, description },
        );
    }

    Ok(())
}
