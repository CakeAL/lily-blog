use proto::post_service_server::PostServiceServer;
use util::{get_db_connection, get_service_addr};

mod dbaccess;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let addr = get_service_addr(util::Service::Post).map_err(|err| {
        tracing::error!("{}", err);
        err
    })?;
    let db = get_db_connection().await.map_err(|err| {
        tracing::error!("{}", err);
        err
    })?;

    let post_service = server::Post::new(db);

    tracing::info!("Post Service runs at: {}", addr);
    tonic::transport::Server::builder()
        .add_service(PostServiceServer::new(post_service))
        .serve(addr.parse()?)
        .await
        .map_err(|err| {
            tracing::error!("{}", err);
            err
        })?;

    Ok(())
}
