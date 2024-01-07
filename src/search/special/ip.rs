use super::SpecialResult;
use anyhow::Result;
use tokio::sync::mpsc::Sender;

pub async fn ip(query: &str, tx: Sender<SpecialResult>) -> Result<()> {
    if query.trim().to_lowercase() == "my ip" {
        let ip = reqwest::Client::new()
            .get("https://ifconfig.me/ip")
            .timeout(std::time::Duration::from_millis(1000))
            .send()
            .await?
            .text()
            .await?;
        tx.send(SpecialResult::IpAddress(ip)).await.unwrap();
    }
    Ok(())
}
