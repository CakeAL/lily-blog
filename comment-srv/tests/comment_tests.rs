use tonic::Request;
use proto::comment_service_client::CommentServiceClient;
use proto::{CreateCommentRequest, GetPostCommentsRequest, ToggleCommentRequest};
use util::get_service_url;

#[tokio::test]
async fn test_create_comment(){
    let client_url = get_service_url(util::Service::Comment).unwrap();
    let mut client = CommentServiceClient::connect(client_url).await.unwrap();
    let request = Request::new(CreateCommentRequest {
        post_id: 1,
        name: "TestName1".into(),
        hashed_email: "73e19518dde2ef0fa4b28895b6a87a8b".into(),
        content: "This is contents of a comment.".into()
    });

    let response = client.create_comment(request).await.unwrap();
    dbg!(response.into_inner());
}

// 没有这个 post_id 应返回错误
#[should_panic]
#[tokio::test]
async fn test_create_comment_panic() {
    let client_url = get_service_url(util::Service::Comment).unwrap();
    let mut client = CommentServiceClient::connect(client_url).await.unwrap();
    let request = Request::new(CreateCommentRequest {
        post_id: i32::MAX,
        name: "TestName1".into(),
        hashed_email: "73e19518dde2ef0fa4b28895b6a87a8b".into(),
        content: "This is contents of a comment.".into()
    });

    let response = client.create_comment(request).await.unwrap();
    dbg!(response.into_inner());
}

#[tokio::test]
async fn test_get_post_comments() {
    let client_url = get_service_url(util::Service::Comment).unwrap();
    let mut client = CommentServiceClient::connect(client_url).await.unwrap();
    let request = Request::new(GetPostCommentsRequest {
        post_id: 1,
    });
    let response = client.get_post_comments(request).await.unwrap();
    dbg!(response);
}

#[tokio::test]
async fn test_toggle_comment() {
    let client_url = get_service_url(util::Service::Comment).unwrap();
    let mut client = CommentServiceClient::connect(client_url).await.unwrap();
    let request = Request::new(ToggleCommentRequest {
        id: 3,
    });
    let response = client.toggle_comment(request).await.unwrap();
    dbg!(response);
}

#[should_panic]
#[tokio::test]
async fn test_toggle_comment_no_such_comment() {
    let client_url = get_service_url(util::Service::Comment).unwrap();
    let mut client = CommentServiceClient::connect(client_url).await.unwrap();
    let request = Request::new(ToggleCommentRequest {
        id: i32::MAX,
    });
    let response = client.toggle_comment(request).await.unwrap();
    dbg!(response);
}

// 应为空 post_id 的评论的 is_del 为 true，返回应为空 vector
#[tokio::test]
async fn test_get_post_comments_with_no_comments() {
    let client_url = get_service_url(util::Service::Comment).unwrap();
    let mut client = CommentServiceClient::connect(client_url).await.unwrap();
    let request = Request::new(GetPostCommentsRequest {
        post_id: 1,
    });
    let response = client.get_post_comments(request).await.unwrap();
    assert_eq!(response.into_inner().comments, vec![]);
}