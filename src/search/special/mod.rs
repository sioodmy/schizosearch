use calculator::calculator;
use dictionary::dictionary;
use ip::ip;
use tokio::{join, sync::mpsc::Sender};

mod calculator;
mod dictionary;
mod ip;

#[derive(Debug)]
pub enum SpecialResult {
    Empty,
    Calculator(String),
    Definition {
        word: String,
        definitions: Vec<String>,
        phonetic: String,
    },
    IpAddress(String),
}

pub async fn special(query: &str, tx: Sender<SpecialResult>) {
    let _ = join!(
        calculator(query, tx.clone()),
        dictionary(query, tx.clone()),
        ip(query, tx.clone())
    );
}
