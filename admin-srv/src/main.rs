use proto::admin_service_server::AdminServiceServer;
use util::{get_db_connection, get_service_addr};

mod dbaccess;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let addr = get_service_addr(util::Service::Admin).map_err(|err| {
        tracing::error!("{}", err);
        err
    })?;
    let db = get_db_connection().await.map_err(|err| {
        tracing::error!("{}", err);
        err
    })?;

    let admin_service = server::Admin::new(db);

    tracing::info!("Admin Service runs at: {}", addr);
    tonic::transport::Server::builder()
        .add_service(AdminServiceServer::new(admin_service))
        .serve(addr.parse()?)
        .await
        .map_err(|err| {
            tracing::error!("{}", err);
            err
        })?;

    Ok(())
}
