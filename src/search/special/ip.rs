use anyhow::Result;
use schizosearch::fetch;
use super::{SpecialResult, SpecialSender};

pub async fn ip(query: &str, tx: SpecialSender) -> Result<()> {
    if query.trim().to_lowercase() == "my ip" {
    let ip = reqwest::Client::new()
        .get("https://ifconfig.me/ip")
        .timeout(std::time::Duration::from_millis(1000))
        .send()
        .await?.text().await?;
        tx.send(Some(SpecialResult::IpAddress(ip))).unwrap();
    }
    Ok(())
}
