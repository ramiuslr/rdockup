mod api;
mod registry_client;

use tower_http::trace::{self, TraceLayer};
use tracing::{Level, info};
// use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // Initialize logging for access logs
    tracing_subscriber::fmt()
        // .with_env_filter(
        //     EnvFilter::try_from_default_env()
        //         .or_else(|_| EnvFilter::try_new("rdockup=error,tower_http=warn"))
        //         .unwrap(),
        // )
        .with_target(false)
        .json()
        .init();

    // Build app with routes from api module and add logging middleware
    let app = api::create_router().layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Starting server on {:?}", listener);
    axum::serve(listener, app).await.unwrap();
}
