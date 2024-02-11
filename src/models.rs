#[derive(sqlx::FromRow)]
pub struct Task {
    pub id: Option<i64>,
    pub title: String,
    pub done: bool,
}
