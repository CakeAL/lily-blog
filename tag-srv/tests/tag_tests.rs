use proto::{tag_service_client::TagServiceClient, CreateTagRequest};
use tonic::Request;
use util::get_service_url;

#[tokio::test]
async fn test_create_tag() {
    let client_url = get_service_url (util::Service::Tag).unwrap();
    let mut client = TagServiceClient::connect(client_url).await.unwrap();
    let request = Request::new(CreateTagRequest {
        name: "tag1".to_string(),
    });

    let reply = client.create_tag(request).await.unwrap();
    dbg!(reply.into_inner());
}
