use std::sync::Arc;

use crate::dbaccess::*;
use proto::post_service_server::PostService;
use proto::{
    CreatePostReply, CreatePostRequest, EditPostReply, EditPostRequest, GetPostReply,
    GetPostRequest, ListPostReply, ListPostRequest, TogglePostReply, TogglePostRequest,
};
use sea_orm::DatabaseConnection;
use tonic::{Request, Response, Status};

const PAGE_SIZE: i32 = 10;

pub struct Post {
    db_conn: Arc<DatabaseConnection>,
}

impl Post {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Self {
            db_conn: Arc::new(db_conn),
        }
    }
}

#[tonic::async_trait]
impl PostService for Post {
    async fn create_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> Result<Response<CreatePostReply>, Status> {
        let CreatePostRequest {
            title,
            tag_id,
            md_path,
            summary,
        } = request.into_inner();
        let summary = match summary {
            None => util::get_summary(&md_path).map_err(|err| Status::internal(err.to_string()))?,
            Some(s) => s,
        };
        let res = insert_new_post(&self.db_conn, title, tag_id, md_path, summary)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        Ok(Response::new(CreatePostReply { id: res }))
    }

    async fn edit_post(
        &self,
        request: Request<EditPostRequest>,
    ) -> Result<Response<EditPostReply>, Status> {
        let r = request.into_inner();
        let summary = match r.summary {
            None => {
                util::get_summary(&r.md_path).map_err(|err| Status::internal(err.to_string()))?
            }
            Some(s) => s,
        };
        let res = update_post(&self.db_conn, r.id, r.title, r.tag_id, r.md_path, summary)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        Ok(Response::new(EditPostReply {
            id: r.id,
            ok: res > 0,
        }))
    }

    async fn list_posts(
        &self,
        request: Request<ListPostRequest>,
    ) -> Result<Response<ListPostReply>, Status> {
        let ListPostRequest {
            page,
            tag_id,
            keyword,
            is_del,
            dateline_range,
        } = request.into_inner();
        let page = page.unwrap_or(0);
        let offset = PAGE_SIZE * page;
        let (start, end) = if let Some(dr) = dateline_range {
            (
                util::timestamp_conversion(dr.start),
                util::timestamp_conversion(dr.end),
            )
        } else {
            (None, None)
        };
        let record_total =
            select_record_total(&self.db_conn, tag_id, keyword.clone(), is_del, start, end)
                .await
                .map_err(|err| Status::internal(err.to_string()))?;
        let page_total = f64::ceil(record_total as f64 / PAGE_SIZE as f64) as i32;

        let res = select_posts(
            &self.db_conn,
            tag_id,
            keyword,
            is_del,
            start,
            end,
            PAGE_SIZE,
            offset,
        )
        .await
        .map_err(|err| Status::internal(err.to_string()))?;
        if res.is_empty() {
            return Err(Status::not_found("no such posts"));
        }

        let posts = res
            .iter()
            .map(model_to_post)
            .collect::<Vec<proto::Post>>();
        Ok(Response::new(ListPostReply {
            page,
            page_total,
            posts,
        }))
    }

    async fn toggle_post(
        &self,
        request: Request<TogglePostRequest>,
    ) -> Result<Response<TogglePostReply>, Status> {
        let TogglePostRequest { id } = request.into_inner();
        let is_del = update_post_del(&self.db_conn, id)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        Ok(Response::new(TogglePostReply { id, is_del }))
    }

    async fn get_post(
        &self,
        request: Request<GetPostRequest>,
    ) -> Result<Response<GetPostReply>, Status> {
        let GetPostRequest {
            id,
            is_del,
            inc_hit,
        } = request.into_inner();
        let post = select_a_post(&self.db_conn, id, is_del, inc_hit)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        let post = post.map(|post| model_to_post(&post));
        Ok(Response::new(GetPostReply { post }))
    }
}

fn model_to_post(post: &entity::entity::post::Model) -> proto::Post {
    proto::Post {
        id: post.id,
        title: post.title.to_owned(),
        tag_id: util::u8_to_tags(post.tag_id.to_owned().unwrap_or_default()), // tag_id 在这儿捏
        summary: post.summary.to_owned(),
        md_path: post.md_path.to_owned(),
        html_path: post.html_path.to_owned(),
        hit: post.hit,
        words_len: post.words_len.unwrap_or(0),
        is_del: post.is_del,
        publish_time: util::datetime_conversion(Some(post.publish_time)),
        update_time: util::datetime_conversion(post.update_time),
    }
}