use crate::home::homepage;
use crate::search::search;
use axum::{routing::get, Router};
use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod home;
mod search;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    let cli = config::Cli::parse();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "schizosearch=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let app = Router::new()
        .route("/", get(homepage))
        .route("/search", get(search))
        .route(
            "/lexend.ttf",
            get(|| async {
                (
                    [
                        ("Content-Type", "font/ttf"),
                        ("Cache-Control", "max-age=31536000"),
                    ],
                    include_bytes!("../assets/lexend.ttf"),
                )
            }),
        );

    let listener = tokio::net::TcpListener::bind(cli.listener).await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
