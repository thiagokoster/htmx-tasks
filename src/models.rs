#[derive(sqlx::FromRow)]
pub struct Task {
    pub id: Option<u64>,
    pub title: String,
    pub done: bool,
}
