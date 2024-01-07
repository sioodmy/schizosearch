use anyhow::{anyhow, Result};
use schizosearch::fetch;
use serde_json::Value;
use tokio::sync::mpsc::Sender;

use super::SpecialResult;

// TODO
// get definition for first word in query,
// which doesnt start with "what", "is", "a", "an" etc

pub async fn dictionary(query: &str, tx: Sender<SpecialResult>) -> Result<()> {
    let mut word: &str = query;

    // dumb way of guessing which word the user is searching for
    // it works tho
    for part in query.trim().split_whitespace().into_iter() {
        match part {
            "what" | "whats" | "is" | "an" | "a" | "definition" | "of" => continue,
            _ => {
                word = part;
                break;
            }
        };
    }
    let json = fetch!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);

    let data: Value = serde_json::from_str(&json)?;

    let mut definitions: Vec<String> = Vec::new();
    if let Some(defs) = data[0]["meanings"][0]["definitions"].as_array() {
        for (i, definition) in defs.iter().enumerate() {
            if i > 5 {
                break;
            }
            if let Some(definition) = definition.get("definition") {
                definitions.push(definition.as_str().unwrap().to_owned());
            }
        }
    }

    let phonetic = data[0]
        .get("phonetic")
        .ok_or(anyhow!("couldnt get phonetic"))?
        .as_str()
        .unwrap()
        .to_owned();

    tx.send(SpecialResult::Definition {
        word: word.to_string(),
        definitions,
        phonetic,
    })
    .await
    .unwrap();

    Ok(())
}
