use crate::handler::post::*;
use crate::model::AppState;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;

pub async fn route_not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}

pub fn post_routes() -> Router<AppState> {
    Router::new()
        .route("/api/get_all", get(search_posts))
        .route("/api/get_post/:id", get(get_single_post))
}
