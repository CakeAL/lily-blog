use crate::model::AppState;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use proto::GetPostRequest;
use serde::Deserialize;
use serde_json::json;
use tokio::fs::File;
use tokio::io;
use tokio::io::AsyncReadExt;

#[derive(Deserialize)]
pub struct SearchParams {
    keyword: Option<String>,
    tag_id: Option<i32>,
    date_range: Option<(i64, i64)>,
    page: Option<i32>,
}

pub async fn search_posts(
    mut state: State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let page = params.page.unwrap_or(1);
    let date_range = util::i64_to_dateline_range(params.date_range);

    // 只查询没有标记为删除的
    let request = tonic::Request::new(proto::ListPostRequest {
        page: Some(page - 1),
        tag_id: params.tag_id,
        keyword: params.keyword,
        is_del: Some(false),
        dateline_range: date_range,
    });
    let proto::ListPostReply {
        posts,
        page,
        page_total,
    } = state
        .post
        .list_posts(request)
        .await
        .map_err(|err| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "message": err.message() })),
            )
        })?
        .into_inner();

    let res = entity::model::ListPostRes {
        page: page + 1,
        page_total,
        posts: posts.into_iter().map(Into::into).collect(),
    };

    Ok((StatusCode::OK, Json(json!(res))))
}

pub async fn get_single_post(
    mut state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let request = tonic::Request::new(GetPostRequest {
        id,
        is_del: Some(false),
        inc_hit: Some(true),
    });
    let proto::GetPostReply { post } = state
        .post
        .get_post(request)
        .await
        .map_err(|err| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "message": err.message() })),
            )
        })?
        .into_inner();

    match post {
        Some(post) => {
            let content = get_content(&post.html_path).await.map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "message": err.to_string() })),
                )
            })?;
            Ok((
                StatusCode::OK,
                Json(json!(entity::model::GetPostRes {
                    post: post.into(),
                    content,
                })),
            ))
        }
        None => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Post not found" })),
        )),
    }
}

async fn get_content(html_path: &str) -> io::Result<String> {
    let mut content = String::new();
    File::open(html_path)
        .await?
        .read_to_string(&mut content)
        .await?;
    Ok(content)
}
