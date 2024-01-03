use anyhow::Result;
use schizosearch::fetch;
use scraper::{Html, Selector};

use super::ResultHtml;

pub async fn duckduckgo(query: &str) -> Result<Vec<ResultHtml>> {
    let html = fetch!("https://html.duckduckgo.com/html/?q={}&kd=-1", query);
    let fragment = Html::parse_document(&html);
    let selector = Selector::parse("div.web-result").unwrap();
    let mut results = Vec::new();
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

        results.push(ResultHtml {
            title,
            link: link.to_owned(),
            description,
        });
    }

    Ok(results)
}
