use crate::handlers::HtmlTemplate;
use crate::models::Task;
use crate::repositories::task;
use askama::Template;
use axum::extract::State;
use axum::response::IntoResponse;
use sqlx::SqlitePool;
use tracing::info;

#[derive(Template)]
#[template(path = "todo.html")]
struct TaskTemplate {
    id: i64,
    title: String,
}

pub async fn create_task(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let new_task = Task {
        id: None,
        title: String::from("Test"),
        done: false,
    };
    let task = task::add(&pool, &new_task).await.unwrap();

    info!("id: {:?}, title: {:?}", task.id, task.title);
    let task_template = TaskTemplate {
        id: task.id.unwrap(),
        title: task.title,
    };

    HtmlTemplate(task_template)
}
