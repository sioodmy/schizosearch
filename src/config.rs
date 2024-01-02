use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = String::from("127.0.0.1:3000"))]
    pub listener: String,
}
