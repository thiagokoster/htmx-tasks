use crate::handlers::HtmlTemplate;
use crate::models::Task;
use crate::repositories::task;
use askama::Template;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Form;
use sqlx::SqlitePool;
use tracing::info;

#[derive(Template, Clone)]
#[template(path = "task.html")]
struct TaskTemplate {
    id: i64,
    title: String,
    done: bool,
}

#[derive(Template)]
#[template(path = "task-list.html")]
struct TaskListTemplate {
    tasks: Vec<TaskTemplate>,
}

#[derive(serde::Deserialize)]
pub struct TaskRequest {
    title: String,
}

pub async fn create_task(
    State(pool): State<SqlitePool>,
    Form(new_task): Form<TaskRequest>,
) -> impl IntoResponse {
    let new_task = Task {
        id: None,
        title: new_task.title,
        done: false,
    };

    let task = task::add(&pool, &new_task).await.unwrap();

    info!("id: {:?}, title: {:?}", task.id, task.title);
    let task_template = TaskTemplate {
        id: task.id.unwrap(),
        title: task.title,
        done: task.done,
    };

    HtmlTemplate(task_template)
}

pub async fn get_all(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let tasks: Vec<TaskTemplate> = task::all(&pool)
        .await
        .unwrap()
        .iter()
        .map(|t| TaskTemplate {
            id: t.id.unwrap(),
            title: t.title.to_string(),
            done: t.done,
        })
        .collect();
    HtmlTemplate(TaskListTemplate { tasks })
}

#[derive(serde::Deserialize)]
pub struct Params {
    task_id: i64,
    done: bool,
}

pub async fn set_done(
    State(pool): State<SqlitePool>,
    Path(params): Path<Params>,
) -> impl IntoResponse {
    let task = task::set_done(&pool, params.task_id, params.done)
        .await
        .unwrap();

    let template = TaskTemplate {
        id: task.id.unwrap(),
        title: task.title,
        done: task.done,
    };

    HtmlTemplate(template)
}
