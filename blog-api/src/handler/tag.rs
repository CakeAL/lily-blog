use crate::model::AppState;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use proto::{GetTagInfoReply, GetTagInfoRequest, ListTagsReply, ListTagsRequest};
use serde::{Deserialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct SearchParams {
    name: Option<String>,
}

pub async fn search_tags(
    mut state: State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let request = tonic::Request::new(ListTagsRequest {
        name: params.name,
        is_del: Some(false),
    });
    let ListTagsReply { tags } = state
        .tag
        .list_tags(request)
        .await
        .map_err(|err| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "message": err.message() })),
            )
        })?
        .into_inner();
    let tags: Vec<entity::model::Tag> = tags.into_iter().map(Into::into).collect();
    Ok((StatusCode::OK, Json(json!(tags))))
}

pub async fn get_tag_info(
    mut state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let request = tonic::Request::new(GetTagInfoRequest {
        id,
        is_del: Some(false),
    });
    let GetTagInfoReply { tag } = state
        .tag
        .get_tag_info(request)
        .await
        .map_err(|err| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "message": err.message() })),
            )
        })?
        .into_inner();
    match tag {
        Some(tag) => Ok((StatusCode::OK, Json(json!(entity::model::Tag::from(tag))))),
        None => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Tag not found" })),
        )),
    }
}
