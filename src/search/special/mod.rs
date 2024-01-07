use calculator::calculator;
use definition::definition;
use ip::ip;
use tokio::{join, sync::mpsc::Sender};

mod calculator;
mod definition;
mod ip;

#[derive(Debug)]
pub enum SpecialResult {
    Empty,
    Calculator(String),
    Definition(String),
    IpAddress(String),
}


pub async fn special(query: &str, tx: Sender<SpecialResult>) {
    let _ = join!(calculator(query, tx.clone()), definition(query, tx.clone()),
        ip(query, tx.clone())
    
    );
}
