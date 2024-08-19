use crate::model::AppState;
use axum::Router;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;

mod model;

#[tokio::main]
async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let app_state = AppState::new().await?;

    let app = Router::new().with_state(app_state).layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    );

    let addr = util::get_service_addr(util::Service::BlogApi)?;
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Blog api listening on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let _ = start().map_err(|err| {
        tracing::error!("{}", err);
    });
}
