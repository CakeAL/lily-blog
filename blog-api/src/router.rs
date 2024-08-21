use crate::handler::{comment::*, post::*, tag::*};
use crate::model::AppState;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::Router;

pub async fn route_not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}

pub fn post_routes() -> Router<AppState> {
    Router::new()
        .route("/api/post/search_posts", get(search_posts))
        .route("/api/post/get_single_post/:id", get(get_single_post))
}

pub fn tag_routes() -> Router<AppState> {
    Router::new()
        .route("/api/tag/search_tags", get(search_tags))
        .route("/api/tag/get_tag_info/:id", get(get_tag_info))
}

pub fn comment_routes() -> Router<AppState> {
    Router::new()
        .route("/api/comment/new_comment", post(new_comment))
        .route(
            "/api/comment/get_post_comments/:post_id",
            get(get_post_comments),
        )
}
