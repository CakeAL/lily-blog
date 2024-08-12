use proto::post_service_client::PostServiceClient;
use proto::{CreatePostRequest, DatelineRange, EditPostRequest, GetPostRequest, ListPostRequest, TogglePostRequest};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono::{Local, TimeZone};
use tonic::Request;
use util::get_service_url;

#[tokio::test]
async fn test_create_post() {
    let client_url = get_service_url(util::Service::Post).unwrap();
    let mut client = PostServiceClient::connect(client_url).await.unwrap();
    let request = Request::new(CreatePostRequest {
        title: "test1".into(),
        tag_id: vec![2, 3, 4],
        md_path: "/Users/cakeal/Desktop/vsc/lily-blog/test_file/test1.md".into(),
        summary: None,
    });
    let response = client.create_post(request).await.unwrap();
    dbg!(response.into_inner());

    let request = Request::new(CreatePostRequest {
        title: "test2".into(),
        tag_id: vec![3, 4],
        md_path: "/Users/cakeal/Desktop/vsc/lily-blog/test_file/test1.md".into(),
        summary: Some("this is a summary".into()),
    });
    let response = client.create_post(request).await.unwrap();
    dbg!(response.into_inner());
}

#[tokio::test]
async fn test_edit_post() {
    let client_url = get_service_url(util::Service::Post).unwrap();
    let mut client = PostServiceClient::connect(client_url).await.unwrap();
    let request = Request::new(EditPostRequest {
        id: 2,
        title: "test1_edited".to_string(),
        tag_id: vec![2, 3, 4, 5],
        md_path: "/Users/cakeal/Desktop/vsc/lily-blog/test_file/test1.md".to_string(),
        summary: None,
    });
    let response = client.edit_post(request).await.unwrap();
    dbg!(response.into_inner());
}

#[tokio::test]
async fn test_list_posts() {
    let client_url = get_service_url(util::Service::Post).unwrap();
    let mut client = PostServiceClient::connect(client_url).await.unwrap();
    // 查询全部
    let request = Request::new(ListPostRequest {
        page: None,
        tag_id: None,
        keyword: None,
        is_del: None,
        dateline_range: None,
    });
    let response = client.list_posts(request).await.unwrap();
    dbg!(response.into_inner());
    // 查询 tag_id = 2
    let request = Request::new(ListPostRequest {
        page: None,
        tag_id: Some(4),
        keyword: None,
        is_del: None,
        dateline_range: None,
    });
    let response = client.list_posts(request).await.unwrap();
    dbg!(response.into_inner());
    // 查询 keyword = test
    let request = Request::new(ListPostRequest {
        page: None,
        tag_id: None,
        keyword: Some("test".into()),
        is_del: None,
        dateline_range: None,
    });
    let response = client.list_posts(request).await.unwrap();
    dbg!(response.into_inner());
    // 查询 is_del = true
    let request = Request::new(ListPostRequest {
        page: None,
        tag_id: None,
        keyword: None,
        is_del: Some(true),
        dateline_range: None,
    });
    let response = client.list_posts(request).await.unwrap();
    dbg!(response.into_inner());
    // 查询 dateline_range = [1723359749, 1723359751]
    let request = Request::new(ListPostRequest {
        page: None,
        tag_id: None,
        keyword: None,
        is_del: None,
        dateline_range: Some(DatelineRange { 
            start: util::datetime_conversion(Some(DateTimeWithTimeZone::from(Local.timestamp_opt(1723359749, 0).unwrap()))),
            end: util::datetime_conversion(Some(DateTimeWithTimeZone::from(Local.timestamp_opt(1723359751, 0).unwrap()))),
        }),
    });
    let response = client.list_posts(request).await.unwrap();
    dbg!(response.into_inner());
}

#[tokio::test]
async fn test_toggle_post() {
    let client_url = get_service_url(util::Service::Post).unwrap();
    let mut client = PostServiceClient::connect(client_url).await.unwrap();
    let request = Request::new(TogglePostRequest { id: 1 });
    let response = client.toggle_post(request).await.unwrap();
    dbg!(response.into_inner());
}

#[tokio::test]
async fn get_post() {
    let client_url = get_service_url(util::Service::Post).unwrap();
    let mut client = PostServiceClient::connect(client_url).await.unwrap();
    // 默认增加点击量
    let request = Request::new(GetPostRequest { id: 1, is_del: None, inc_hit: None });
    let response = client.get_post(request).await.unwrap();
    dbg!(response.into_inner());
    // 设置不增加点击量
    let request = Request::new(GetPostRequest { id: 2, is_del: None, inc_hit: Some(false) });
    let response = client.get_post(request).await.unwrap();
    dbg!(response.into_inner());
    // 查询 id 3，条件 is_del 为 true，会返回 None，没有这个 post
    let request = Request::new(GetPostRequest { id: 3, is_del: Some(true), inc_hit: None });
    let response = client.get_post(request).await.unwrap();
    assert_eq!(response.into_inner().post, None);
}