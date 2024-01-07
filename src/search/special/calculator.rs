
use calculator_rs::Calculate;
use tokio::sync::mpsc::Sender;

use super::SpecialResult;

pub async fn calculator(query: &str, tx: Sender<SpecialResult>) {
        if let Ok(result) = query.calculate(){
            let data = format!("{} = {}", &query, result);
            tx.send(SpecialResult::Calculator(data)).await.unwrap();
        }
}
