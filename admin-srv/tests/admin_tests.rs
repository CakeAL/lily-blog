use proto::admin_service_client::AdminServiceClient;
use proto::get_admin_request::{ByAuth, ById};
use proto::{
    AdminExistsRequest, CreateAdminRequest, EditAdminRequest, GetAdminRequest, ListAdminRequest,
    ToggleAdminRequest,
};
use tonic::Request;
use util::get_service_url;

#[tokio::test]
async fn test_create_admin() {
    let client_url = get_service_url(util::Service::Admin).unwrap();
    let mut client = AdminServiceClient::connect(client_url).await.unwrap();
    let request = Request::new(CreateAdminRequest {
        email: "cakeal@qq.com".to_string(),
        password: "12345678".to_string(),
    });
    let reply = client.create_admin(request).await.unwrap();
    dbg!(reply.into_inner());
}

#[tokio::test]
async fn test_list_admin() {
    let client_url = get_service_url(util::Service::Admin).unwrap();
    let mut client = AdminServiceClient::connect(client_url).await.unwrap();
    let request = Request::new(ListAdminRequest {
        email: Some("qq".to_string()),
        is_del: None,
    });
    let reply = client.list_admin(request).await.unwrap();
    dbg!(reply.into_inner());

    // 查询已删除的 Admin
    let request = Request::new(ListAdminRequest {
        email: None,
        is_del: Some(true),
    });
    let reply = client.list_admin(request).await.unwrap();
    dbg!(reply.into_inner());
}

#[should_panic]
#[tokio::test]
async fn test_edit_admin() {
    let client_url = get_service_url(util::Service::Admin).unwrap();
    let mut client = AdminServiceClient::connect(client_url).await.unwrap();
    let request = Request::new(EditAdminRequest {
        id: 1,
        email: "cakeal@qq.com".to_string(),
        password: "12345678".to_string(),
        new_password: Some("87654321".to_string()),
    });
    let reply = client.edit_admin(request).await.unwrap();
    dbg!(reply.into_inner());

    // 密码错误
    let request = Request::new(EditAdminRequest {
        id: 1,
        email: "cakeal@qq.com".to_string(),
        password: "12345678".to_string(),
        new_password: Some("87654321".to_string()),
    });
    let reply = client.edit_admin(request).await;
    dbg!(reply.unwrap());
}

#[tokio::test]
async fn test_toggle_admin() {
    let client_url = get_service_url(util::Service::Admin).unwrap();
    let mut client = AdminServiceClient::connect(client_url).await.unwrap();
    let request = Request::new(ToggleAdminRequest { id: 1 });
    let reply = client.toggle_admin(request).await.unwrap();
    dbg!(reply.into_inner());

    // 已删除的 Admin 再次 toggle_admin
    let request = Request::new(ToggleAdminRequest { id: 1 });
    let reply = client.toggle_admin(request).await.unwrap();
    dbg!(reply.into_inner());
}

#[tokio::test]
async fn test_admin_exists() {
    let client_url = get_service_url(util::Service::Admin).unwrap();
    let mut client = AdminServiceClient::connect(client_url).await.unwrap();
    let request = Request::new(AdminExistsRequest {
        condition: Some(proto::admin_exists_request::Condition::Email(
            "cakeal@qq.com".to_string(),
        )),
    });
    let reply = client.admin_exists(request).await.unwrap();
    dbg!(reply.into_inner());

    let request = Request::new(AdminExistsRequest {
        condition: Some(proto::admin_exists_request::Condition::Id(1)),
    });
    let reply = client.admin_exists(request).await.unwrap();
    dbg!(reply.into_inner());
}

#[tokio::test]
async fn test_get_admin() {
    let client_url = get_service_url(util::Service::Admin).unwrap();
    let mut client = AdminServiceClient::connect(client_url).await.unwrap();
    let request = Request::new(GetAdminRequest {
        condition: Some(proto::get_admin_request::Condition::ById(ById {
            id: 1,
            is_del: Some(false),
        })),
    });
    let reply = client.get_admin(request).await.unwrap();
    dbg!(reply.into_inner());

    let request = Request::new(GetAdminRequest {
        condition: Some(proto::get_admin_request::Condition::ByAuth(ByAuth {
            email: "cakeal@qq.com".to_string(),
            password: "87654321".to_string(),
        }))
    });
    let reply = client.get_admin(request).await.unwrap();
    dbg!(reply.into_inner());
}
