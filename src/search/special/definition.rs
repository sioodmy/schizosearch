use anyhow::Result;
use schizosearch::fetch;
use serde_json::Value;
use tokio::sync::mpsc::Sender;

use super::SpecialResult;

pub async fn definition(query: &str, tx: Sender<SpecialResult>) -> Result<()> {
    let json = fetch!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", query);

    let data: Value = serde_json::from_str(&json)?;

    if let Some(definition) = data[0]["meanings"][0]["definitions"][0]["definition"].as_str() {
        println!("{}", definition);
        tx.send(SpecialResult::Definition(definition.to_owned()))
            .await
            .unwrap();
    }

    Ok(())
}
