use tonic::Request;
use proto::{CreatePostRequest, EditPostRequest};
use proto::post_service_client::PostServiceClient;
use util::get_service_url;


#[tokio::test]
async fn test_create_post() {
    let client_url = get_service_url(util::Service::Post).unwrap();
    let mut client = PostServiceClient::connect(client_url).await.unwrap();
    let request = Request::new(CreatePostRequest {
        title: "test1".into(),
        tag_id: vec![2,3,4],
        md_path : "/Users/cakeal/Desktop/vsc/lily-blog/test_file/test1.md".into(),
        summary : None,
    });
    let response = client.create_post(request).await.unwrap();
    dbg!(response.into_inner());

    let request = Request::new(CreatePostRequest {
        title: "test2".into(),
        tag_id: vec![3,4],
        md_path : "/Users/cakeal/Desktop/vsc/lily-blog/test_file/test1.md".into(),
        summary : Some("this is a summary".into()),
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
        tag_id: vec![2,3,4,5],
        md_path: "/Users/cakeal/Desktop/vsc/lily-blog/test_file/test1.md".to_string(),
        summary: None,
    });
    let response = client.edit_post(request).await.unwrap();
    dbg!(response.into_inner());
}