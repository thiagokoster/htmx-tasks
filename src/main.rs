use anyhow::Context;
use axum::{routing, Router};
use tracing::info;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

use handlers::app::home;

mod handlers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging();
    info!("Initializing router...");

    let url = "127.0.0.1:3000";
    let router = Router::new().route("/", routing::get(home::handler));

    let addr = tokio::net::TcpListener::bind(url).await.unwrap();

    info!("Listening on {}", url);
    axum::serve(addr, router.into_make_service())
        .await
        .context("Error while starting the server")?;

    Ok(())
}

fn init_logging() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sandbox=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
