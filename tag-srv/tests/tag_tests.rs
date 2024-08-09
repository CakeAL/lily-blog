use proto::{
    tag_exists_request::Condition, tag_service_client::TagServiceClient, CreateTagRequest,
    EditTagRequest, GetTagInfoRequest, ListTagsRequest, TagExistsRequest, ToggleTagRequest,
};
use tonic::Request;
use util::get_service_url;

#[tokio::test]
async fn test_create_tag() {
    let client_url = get_service_url(util::Service::Tag).unwrap();
    let mut client = TagServiceClient::connect(client_url).await.unwrap();
    let request = Request::new(CreateTagRequest {
        name: "tag1".to_string(),
    });

    let reply = client.create_tag(request).await.unwrap();
    dbg!(reply.into_inner());
}

#[tokio::test]
async fn test_edit_tag() {
    let client_url = get_service_url(util::Service::Tag).unwrap();
    let mut client = TagServiceClient::connect(client_url).await.unwrap();
    let request = Request::new(EditTagRequest {
        id: 234243,
        name: "tag1_changed".to_string(),
    });
    let reply = client.edit_tag(request).await.unwrap();
    // rows_affected 应该是 0
    dbg!(reply.into_inner());

    let request = Request::new(EditTagRequest {
        id: 2,
        name: "tag1".to_string(),
    });
    let reply = client.edit_tag(request).await;
    // tag1 已存在，应为 err
    assert_eq!(reply.is_err(), true);

    let request = Request::new(EditTagRequest {
        id: 2,
        name: "tag2".to_string(),
    });
    let reply = client.edit_tag(request).await.unwrap();
    dbg!(reply.into_inner());
}

#[tokio::test]
async fn test_list_tags() {
    let client_url = get_service_url(util::Service::Tag).unwrap();
    let mut client = TagServiceClient::connect(client_url).await.unwrap();
    let request = Request::new(ListTagsRequest {
        name: Some("tag".to_string()),
        is_del: None,
    });
    let reply = client.list_tags(request).await.unwrap();
    dbg!(reply.into_inner()); // 应返回 tag2, tag3, tag4, tag5

    let request = Request::new(ListTagsRequest {
        name: None,
        is_del: Some(false),
    });
    let reply = client.list_tags(request).await.unwrap();
    dbg!(reply.into_inner()); // 应返回 tag2, tag3, tag5

    let request = Request::new(ListTagsRequest {
        name: None,
        is_del: Some(true),
    });
    let reply = client.list_tags(request).await.unwrap();
    dbg!(reply.into_inner()); // 应返回 tag4

    let request = Request::new(ListTagsRequest {
        name: Some("5".to_string()),
        is_del: Some(false),
    });
    let reply = client.list_tags(request).await.unwrap();
    dbg!(reply.into_inner()); // 应返回 tag5
}

#[tokio::test]
async fn test_toggle_tag() {
    let client_url = get_service_url(util::Service::Tag).unwrap();
    let mut client = TagServiceClient::connect(client_url).await.unwrap();
    let req = Request::new(ToggleTagRequest { id: 2 });
    let reply = client.toggle_tag(req).await.unwrap();
    dbg!(reply.into_inner());
}

#[tokio::test]
async fn test_tag_exists() {
    let client_url = get_service_url(util::Service::Tag).unwrap();
    let mut client = TagServiceClient::connect(client_url).await.unwrap();
    let request = Request::new(TagExistsRequest {
        condition: Some(Condition::Name("tag2".to_string())),
    });
    let reply = client.tag_exists(request).await.unwrap();
    assert_eq!(reply.into_inner().exists, true);

    let request = Request::new(TagExistsRequest {
        condition: Some(Condition::Id(2)),
    });
    let reply = client.tag_exists(request).await.unwrap();
    assert_eq!(reply.into_inner().exists, true);

    let request = Request::new(TagExistsRequest {
        condition: Some(Condition::Id(2222)),
    });
    let reply = client.tag_exists(request).await.unwrap();
    assert_eq!(reply.into_inner().exists, false);

    let request = Request::new(TagExistsRequest {
        condition: Some(Condition::Id(3)),
    });
    let reply = client.tag_exists(request).await.unwrap();
    assert_eq!(reply.into_inner().exists, true);
}

#[tokio::test]
async fn test_get_tag_info() {
    let client_url = get_service_url(util::Service::Tag).unwrap();
    let mut client = TagServiceClient::connect(client_url).await.unwrap();

    let request = Request::new(GetTagInfoRequest {
        id: 2,
        is_del: None,
    });
    let reply = client.get_tag_info(request).await.unwrap();
    dbg!(reply.into_inner()); // 应返回 tag2

    let request = Request::new(GetTagInfoRequest {
        id: 4,
        is_del: Some(true),
    });
    let reply = client.get_tag_info(request).await.unwrap();
    dbg!(reply.into_inner()); // 应返回 tag4

    let request = Request::new(GetTagInfoRequest {
        id: 2,
        is_del: Some(true),
    });
    let reply = client.get_tag_info(request).await.unwrap();
    dbg!(reply.into_inner()); // 2 is_del 是 false，应该返回 None
}
