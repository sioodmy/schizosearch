use calculator::calculator;
use definition::definition;
use ip::ip;
use std::sync::mpsc::Sender;
use tokio::join;

mod calculator;
mod definition;
mod ip;

pub type SpecialSender = Sender<Option<SpecialResult>>;
#[derive(Debug)]
pub enum SpecialResult {
    Calculator(String),
    Definition(String),
    IpAddress(String),
}

pub async fn special(query: &str, tx: SpecialSender) {
    let _ = join!(calculator(query, tx.clone()), definition(query, tx.clone()),
        ip(query, tx.clone())
    
    );
}
