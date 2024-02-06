use askama::Template;
use axum::response::IntoResponse;

use crate::handlers::HtmlTemplate;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

pub async fn handler() -> impl IntoResponse {
    HtmlTemplate(HomeTemplate {})
}
