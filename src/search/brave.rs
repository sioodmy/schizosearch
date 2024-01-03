use anyhow::Result;
use schizosearch::fetch;
use scraper::{Html, Selector};

use super::ResultHtml;

pub async fn brave(query: &str) -> Result<Vec<ResultHtml>> {
    let html = fetch!(
        "https://search.brave.com/search?q={}&nfpr=1&spellcheck=0",
        query
    );
    let fragment = Html::parse_document(&html);
    let selector = Selector::parse("div.snippet").unwrap();
    let mut results = Vec::new();
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

        results.push(ResultHtml {
            title,
            link: link.to_owned(),
            description,
        });
    }

    Ok(results)
}
