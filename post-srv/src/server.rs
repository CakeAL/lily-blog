use std::sync::Arc;

use crate::dbaccess::*;
use proto::post_service_server::PostService;
use proto::{
    CreatePostReply, CreatePostRequest, EditPostReply, EditPostRequest, GetPostReply,
    GetPostRequest, ListPostReply, ListPostRequest, TogglePostReply, TogglePostRequest,
};
use sea_orm::DatabaseConnection;
use tonic::{Request, Response, Status};

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
        let res = update_post(&self.db_conn, r.id, r.title, r.tag_id, r.md_path, summary).await
            .map_err(|err| Status::internal(err.to_string()))?;
        Ok(Response::new(EditPostReply { id: r.id, ok: res > 0 }))
    }

    async fn list_posts(
        &self,
        request: Request<ListPostRequest>,
    ) -> Result<Response<ListPostReply>, Status> {
        todo!()
    }

    async fn toggle_post(
        &self,
        request: Request<TogglePostRequest>,
    ) -> Result<Response<TogglePostReply>, Status> {
        todo!()
    }

    async fn get_post(
        &self,
        request: Request<GetPostRequest>,
    ) -> Result<Response<GetPostReply>, Status> {
        todo!()
    }
}
