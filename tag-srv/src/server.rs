use std::sync::Arc;

use proto::{
    tag_exists_request::Condition, tag_service_server::*, CreateTagReply, CreateTagRequest,
    EditTagReply, EditTagRequest, GetTagInfoReply, GetTagInfoRequest, ListTagsReply,
    ListTagsRequest, TagExistsReply, TagExistsRequest, ToggleTagReply, ToggleTagRequest,
};
use sea_orm::DatabaseConnection;
use tonic::{Request, Response, Status};

use crate::dbaccess::*;

pub struct Tag {
    db_conn: Arc<DatabaseConnection>,
}

impl Tag {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Tag {
            db_conn: Arc::new(db_conn),
        }
    }
}

#[tonic::async_trait]
impl TagService for Tag {
    async fn create_tag(
        &self,
        request: Request<CreateTagRequest>,
    ) -> Result<Response<CreateTagReply>, Status> {
        let CreateTagRequest { name } = request.into_inner();

        // whether the name exists
        let exist_request = Request::new(TagExistsRequest {
            condition: Some(Condition::Name(name.clone())),
        });
        let exist_response = self.tag_exists(exist_request).await?.into_inner();
        if exist_response.exists {
            return Err(Status::already_exists("Tag already exists"));
        }

        // create the tag
        let res = insert_new_tag(&self.db_conn, &name)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        Ok(Response::new(CreateTagReply { id: res }))
    }

    async fn edit_tag(
        &self,
        request: Request<EditTagRequest>,
    ) -> Result<Response<EditTagReply>, Status> {
        let EditTagRequest { id, name } = request.into_inner();

        // whether the tag exists
        let exist_request = Request::new(TagExistsRequest {
            condition: Some(Condition::Name(name.clone())),
        });
        let exist_response = self.tag_exists(exist_request).await?.into_inner();
        if exist_response.exists {
            return Err(Status::already_exists("Tag already exists"));
        }

        // edit the tag
        let rows_affected = update_tag(&self.db_conn, id, &name)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        Ok(Response::new(EditTagReply {
            id,
            ok: rows_affected > 0,
        }))
    }

    async fn list_tags(
        &self,
        request: Request<ListTagsRequest>,
    ) -> Result<Response<ListTagsReply>, Status> {
        let ListTagsRequest { name, is_del } = request.into_inner();
        let res = select_tags(&self.db_conn, &name, &is_del)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;

        // res is empty
        if res.is_empty() {
            return Err(Status::not_found("no such tag"));
        }

        let mut tags = Vec::with_capacity(res.len());
        for tag in res {
            tags.push(proto::Tag {
                name: tag.name,
                id: tag.id,
                is_del: tag.is_del,
            });
        }

        Ok(Response::new(ListTagsReply { tags }))
    }

    async fn toggle_tag(
        &self,
        request: Request<ToggleTagRequest>,
    ) -> Result<Response<ToggleTagReply>, Status> {
        let ToggleTagRequest { id } = request.into_inner();

        let is_del = update_tag_del(&self.db_conn, id)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;

        Ok(Response::new(ToggleTagReply { id, is_del }))
    }

    async fn tag_exists(
        &self,
        request: Request<TagExistsRequest>,
    ) -> Result<Response<TagExistsReply>, Status> {
        let request = request.into_inner();
        let condition = request
            .condition
            .ok_or(tonic::Status::invalid_argument("Invalid argument"))?;
        let res = match condition {
            Condition::Id(id) => select_tag_exists_by_id(&self.db_conn, id).await,
            Condition::Name(name) => select_tag_exists_by_name(&self.db_conn, &name).await,
        }
        .map_err(|err| Status::internal(err.to_string()))?;
        Ok(Response::new(TagExistsReply { exists: res > 0 }))
    }

    async fn get_tag_info(
        &self,
        request: Request<GetTagInfoRequest>,
    ) -> Result<Response<GetTagInfoReply>, Status> {
        let GetTagInfoRequest { id, is_del } = request.into_inner();

        let res = select_tag_info(&self.db_conn, id, &is_del)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;

        let tag = match res {
            Some(tag) => Some(proto::Tag {
                name: tag.name,
                id: tag.id,
                is_del: tag.is_del,
            }),
            None => None,
        };
        Ok(Response::new(GetTagInfoReply { tag }))
    }
}
