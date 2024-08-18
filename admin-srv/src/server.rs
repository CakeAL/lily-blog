use crate::dbaccess::*;
use proto::admin_service_server::AdminService;
use proto::{
    AdminExistsReply, AdminExistsRequest, CreateAdminReply, CreateAdminRequest, EditAdminReply,
    EditAdminRequest, GetAdminReply, GetAdminRequest, ListAdminReply, ListAdminRequest,
    ToggleAdminReply, ToggleAdminRequest,
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use util::password;

pub struct Admin {
    db_conn: Arc<DatabaseConnection>,
}

impl Admin {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Admin {
            db_conn: Arc::new(db_conn),
        }
    }
}

#[tonic::async_trait]
impl AdminService for Admin {
    async fn create_admin(
        &self,
        request: Request<CreateAdminRequest>,
    ) -> Result<Response<CreateAdminReply>, Status> {
        let CreateAdminRequest { email, password } = request.into_inner();
        let AdminExistsReply { exists } = self
            .admin_exists(Request::new(AdminExistsRequest {
                condition: Some(proto::admin_exists_request::Condition::Email(email.clone())),
            }))
            .await?
            .into_inner();
        if exists {
            return Err(Status::already_exists("该邮箱已存在"));
        }
        let pwd = password::hash(&password).map_err(Status::internal)?;
        let id = insert_admin(&self.db_conn, &email, &pwd)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        Ok(Response::new(CreateAdminReply { id }))
    }

    async fn list_admin(
        &self,
        request: Request<ListAdminRequest>,
    ) -> Result<Response<ListAdminReply>, Status> {
        let ListAdminRequest { email, is_del } = request.into_inner();
        let res = select_admins(&self.db_conn, &email, &is_del)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        let admins = res
            .iter()
            .map(|model| proto::Admin {
                id: model.id,
                email: model.email.to_owned(),
                password: None,
                is_del: model.is_del,
            })
            .collect::<Vec<proto::Admin>>();
        Ok(Response::new(ListAdminReply { admins }))
    }

    async fn edit_admin(
        &self,
        request: Request<EditAdminRequest>,
    ) -> Result<Response<EditAdminReply>, Status> {
        let EditAdminRequest {
            id,
            email,
            password,
            new_password,
        } = request.into_inner();

        // 获取原来的管理员信息
        let admin = select_admin_by_email(&self.db_conn, &email)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        let admin = admin.ok_or(Status::invalid_argument("不存在的用户"))?;
        // 验证旧密码是否相同
        let is_verify = password::verify(&password, &admin.password).map_err(Status::internal)?;
        if !is_verify {
            return Err(Status::unauthenticated("旧密码不正确"));
        }
        // 新密码
        let new_password = new_password.ok_or(Status::invalid_argument("请设定新密码"))?;
        let hashed_new_pwd = password::hash(&new_password).map_err(Status::internal)?;
        // 更新
        let rows_affected = update_admin_pwd(&self.db_conn, id, &hashed_new_pwd)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        Ok(Response::new(EditAdminReply {
            id,
            ok: rows_affected > 0,
        }))
    }

    async fn toggle_admin(
        &self,
        request: Request<ToggleAdminRequest>,
    ) -> Result<Response<ToggleAdminReply>, Status> {
        let ToggleAdminRequest { id } = request.into_inner();
        let is_del = update_admin_del(&self.db_conn, id)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        Ok(Response::new(ToggleAdminReply { id, is_del }))
    }

    async fn admin_exists(
        &self,
        request: Request<AdminExistsRequest>,
    ) -> Result<Response<AdminExistsReply>, Status> {
        let AdminExistsRequest { condition } = request.into_inner();
        let condition = condition.ok_or(Status::invalid_argument("请指定条件"))?;
        let count = match condition {
            proto::admin_exists_request::Condition::Email(email) => {
                count_admin_by_email(&self.db_conn, &email).await
            }
            proto::admin_exists_request::Condition::Id(id) => {
                count_admin_by_id(&self.db_conn, id).await
            }
        }
        .map_err(|err| Status::internal(err.to_string()))?;
        Ok(Response::new(AdminExistsReply { exists: count > 0 }))
    }

    async fn get_admin(
        &self,
        request: Request<GetAdminRequest>,
    ) -> Result<Response<GetAdminReply>, Status> {
        let GetAdminRequest { condition } = request.into_inner();
        let condition = condition.ok_or(Status::invalid_argument("请指定条件"))?;
        let reply = match condition {
            proto::get_admin_request::Condition::ByAuth(ba) => {
                let admin = select_admin_by_email(&self.db_conn, &ba.email)
                    .await
                    .map_err(|err| Status::internal(err.to_string()))?;
                if let Some(admin) = admin {
                    let is_verify = password::verify(&ba.password, &admin.password)
                        .map_err(Status::internal)?;
                    if !is_verify {
                        return Err(Status::invalid_argument("用户名/密码错误"));
                    } else {
                        GetAdminReply {
                            admin: Some(proto::Admin {
                                id: admin.id,
                                email: admin.email,
                                password: None,
                                is_del: admin.is_del,
                            }),
                        }
                    }
                } else {
                    return Err(Status::invalid_argument("用户名/密码错误"));
                }
            }
            proto::get_admin_request::Condition::ById(bi) => {
                let admin = select_admin_by_id(&self.db_conn, bi.id, bi.is_del)
                    .await
                    .map_err(|err| Status::internal(err.to_string()))?;
                if let Some(admin) = admin {
                    GetAdminReply {
                        admin: Some(proto::Admin {
                            id: admin.id,
                            email: admin.email,
                            password: None,
                            is_del: admin.is_del,
                        }),
                    }
                } else {
                    return Err(Status::invalid_argument("不存在的用户"));
                }
            }
        };
        Ok(Response::new(reply))
    }
}
