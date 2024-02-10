use sqlx::SqlitePool;
use tracing::info;

use crate::models::Task;

pub async fn add(pool: &SqlitePool, task: &Task) -> Result<(), sqlx::Error> {
    info!("Adding task {}", task.title);
    let result = sqlx::query!(
        r#"
    INSERT INTO task (title) VALUES ($1)
    RETURNING id, title, done
    "#,
        task.title
    )
    .fetch_one(pool)
    .await;
    //TODO: Return the added task
    Ok(())
}
