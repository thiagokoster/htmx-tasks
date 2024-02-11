use sqlx::SqlitePool;
use tracing::info;

use crate::models::Task;

pub async fn add(pool: &SqlitePool, task: &Task) -> Result<(Task), sqlx::Error> {
    info!("Adding task {}", task.title);
    let result = sqlx::query_as!(
        Task,
        r#"
    INSERT INTO task (title) VALUES ($1)
    RETURNING id, title, done
    "#,
        task.title
    )
    .fetch_one(pool)
    .await?;
    Ok(result)
}

pub async fn all(pool: &SqlitePool) -> Result<(Vec<Task>), sqlx::Error> {
    let result = sqlx::query_as!(
        Task,
        r#"
    SELECT id, title, done
    FROM task
    "#
    )
    .fetch_all(pool)
    .await?;
    Ok(result)
}

pub async fn set_done(pool: &SqlitePool, id: i64, done: bool) -> Result<(Task), sqlx::Error> {
    let result = sqlx::query_as!(
        Task,
        r#"
    UPDATE task
    SET done = $1 
    WHERE id = $2
    RETURNING id, title, done
    "#,
        done,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(result)
}
