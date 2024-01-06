use std::sync::mpsc::Sender;

use calculator_rs::Calculate;

use super::SpecialResult;

pub async fn calculator(query: &str, tx: Sender<Option<SpecialResult>>) {
    match query.calculate() {
        Ok(result) => {
            let data = format!("{} = {}", &query, result);
            tx.send(Some(SpecialResult::Calculator(data))).unwrap();
        }
        Err(_) => tx.send(None).unwrap(),
    }
}
