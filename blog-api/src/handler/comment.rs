use crate::model::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use proto::{
    CreateCommentReply, CreateCommentRequest, GetPostCommentsReply, GetPostCommentsRequest,
};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct NewCommentJson {
    post_id: i32,
    name: String,
    hashed_email: String,
    content: String,
}

pub async fn new_comment(
    mut state: State<AppState>,
    Json(new_comment): Json<NewCommentJson>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let request = tonic::Request::new(CreateCommentRequest {
        post_id: new_comment.post_id,
        name: new_comment.name,
        hashed_email: new_comment.hashed_email,
        content: new_comment.content,
    });
    let CreateCommentReply { id } = state
        .comment
        .create_comment(request)
        .await
        .map_err(|err| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "message": err.message() })),
            )
        })?
        .into_inner();
    Ok((StatusCode::OK, Json(json!({ "id": id }))))
}

pub async fn get_post_comments(
    mut state: State<AppState>,
    Path(post_id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let request = tonic::Request::new(GetPostCommentsRequest { post_id });
    let GetPostCommentsReply { comments } = state
        .comment
        .get_post_comments(request)
        .await
        .map_err(|err| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "message": err.message() })),
            )
        })?
        .into_inner();
    let comments: Vec<entity::model::Comment> = comments.into_iter().map(Into::into).collect();
    Ok((StatusCode::OK, Json(json!(comments))))
}
