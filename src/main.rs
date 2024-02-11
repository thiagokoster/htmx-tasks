use anyhow::Context;
use axum::{routing, Router};
use dotenvy::dotenv;
use handlers::api::hello;
use handlers::api::tasks;
use handlers::app::home;
use sqlx::{self, SqlitePool};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod models;
mod repositories;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().expect(".env file not found");
    init_logging();
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&db_url).await?;

    info!("Initializing router...");

    let url = "127.0.0.1:3000";
    let assets_path = std::env::current_dir().unwrap();

    let api = Router::new()
        .route("/hello", routing::get(hello::handler))
        .route("/tasks", routing::post(tasks::create_task))
        .route("/tasks", routing::get(tasks::get_all))
        .route("/tasks/:task_id/done/:done", routing::put(tasks::set_done))
        .with_state(pool);
    let app = Router::new().route("/", routing::get(home::handler));

    let router = Router::new().nest("/", app).nest("/api", api).nest_service(
        "/assets",
        ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
    );

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
