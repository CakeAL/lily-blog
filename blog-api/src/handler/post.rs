use crate::model::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct PageParam {
    pub page: Option<i32>,
}

pub async fn get_all_posts(
    mut state: State<AppState>,
    Query(param): Query<PageParam>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let page = param.page.unwrap_or(1);

    // 只查询没有标记为删除的
    let request = tonic::Request::new(proto::ListPostRequest {
        page: Some(page - 1),
        tag_id: None,
        keyword: None,
        is_del: Some(false),
        dateline_range: None,
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
