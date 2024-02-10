use crate::models::Task;
use crate::repositories::task;
use axum::extract::State;
use axum::response::IntoResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn create_task(State(pool): State<SqlitePool>) -> impl IntoResponse {
    info!("is connected: {}", !pool.is_closed());
    let new_task = Task {
        id: None,
        title: String::from("Test"),
        done: false,
    };
    task::add(&pool, &new_task).await;

    "test"
}
