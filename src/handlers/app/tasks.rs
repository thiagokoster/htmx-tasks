use crate::{repositories::task, tasks::TaskTemplate};
use askama::Template;
use axum::{extract::State, response::IntoResponse};
use sqlx::SqlitePool;

use crate::handlers::HtmlTemplate;

#[derive(Template)]
#[template(path = "tasks.html")]
struct TasksTemplate {
    tasks: Vec<TaskTemplate>,
}

pub async fn handler(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let tasks = task::all(&pool)
        .await
        .unwrap()
        .iter()
        .map(|t| TaskTemplate {
            id: t.id.unwrap(),
            title: t.title.to_string(),
            done: t.done,
        })
        .collect();

    HtmlTemplate(TasksTemplate { tasks })
}
