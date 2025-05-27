mod api;
mod registry_client;

// use axum::Router;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{filter::LevelFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize logging for access logs
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(LevelFilter::INFO)
        .init();

    // Build app with routes from api module and add logging middleware
    let app = api::create_router().layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
