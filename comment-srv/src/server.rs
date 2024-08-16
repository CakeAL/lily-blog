use std::sync::Arc;

use proto::{
    comment_service_server::CommentService, CreateCommentReply, CreateCommentRequest,
    GetPostCommentsReply, GetPostCommentsRequest, ToggleCommentReply, ToggleCommentRequest,
};
use sea_orm::DatabaseConnection;
use tonic::{Request, Response, Status};

use crate::dbaccess::*;

pub struct Comment {
    db_conn: Arc<DatabaseConnection>,
}

impl Comment {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Comment {
            db_conn: Arc::new(db_conn),
        }
    }
}

#[tonic::async_trait]
impl CommentService for Comment {
    async fn create_comment(
        &self,
        request: Request<CreateCommentRequest>,
    ) -> Result<Response<CreateCommentReply>, Status> {
        let CreateCommentRequest {
            post_id,
            name,
            hashed_email,
            content,
        } = request.into_inner();
        let id = insert_comment(&self.db_conn, post_id, name, hashed_email, content)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;

        Ok(Response::new(CreateCommentReply { id }))
    }

    async fn get_post_comments(
        &self,
        request: Request<GetPostCommentsRequest>,
    ) -> Result<Response<GetPostCommentsReply>, Status> {
        let GetPostCommentsRequest { post_id } = request.into_inner();
        let res = select_comments(&self.db_conn, post_id)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        let comments = res
            .iter()
            .map(|comment| proto::Comment {
                id: comment.id,
                post_id: comment.post_id,
                name: comment.name.to_owned(),
                hashed_email: comment.hashed_email.to_owned().unwrap_or_default(),
                content: comment.content.to_owned().unwrap_or_default(),
                created_at: util::datetime_conversion(Some(comment.created_at)),
                is_del: comment.is_del,
            })
            .collect::<Vec<proto::Comment>>();
        Ok(Response::new(GetPostCommentsReply { comments }))
    }

    async fn toggle_comment(
        &self,
        request: Request<ToggleCommentRequest>,
    ) -> Result<Response<ToggleCommentReply>, Status> {
        let ToggleCommentRequest { id } = request.into_inner();
        let res = update_comment_del(&self.db_conn, id)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        Ok(Response::new(ToggleCommentReply { id, is_del: res }))
    }
}
