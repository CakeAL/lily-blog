use proto::comment_service_client::CommentServiceClient;
use proto::post_service_client::PostServiceClient;
use proto::tag_service_client::TagServiceClient;
use util::{get_service_url, Service};

#[derive(Clone)]
pub struct AppState {
    pub comment: CommentServiceClient<tonic::transport::Channel>,
    pub post: PostServiceClient<tonic::transport::Channel>,
    pub tag: TagServiceClient<tonic::transport::Channel>,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let comment = CommentServiceClient::connect(get_service_url(Service::Comment)?).await?;
        let post = PostServiceClient::connect(get_service_url(Service::Post)?).await?;
        let tag = TagServiceClient::connect(get_service_url(Service::Tag)?).await?;
        Ok(Self { comment, post, tag })
    }
}
