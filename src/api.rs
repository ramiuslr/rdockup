use axum::{
    Router,
    extract::Json,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
};
use serde::Deserialize;

use crate::registry_client;

#[derive(Deserialize)]
struct TagRequest {
    image: String,
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
}

fn validate_token(token: &str) -> bool {
    token == "my-secret-token"
}

pub fn create_router() -> Router {
    Router::new().route("/api/tags", post(get_tags_handler))
}

async fn get_tags_handler(
    headers: HeaderMap,
    Json(payload): Json<TagRequest>,
) -> impl IntoResponse {
    // Extract Bearer token from Authorization header
    let token = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .unwrap_or("");

    if !validate_token(token) {
        return (
            StatusCode::UNAUTHORIZED,
            [("content-type", "text/plain")],
            "Invalid or missing token".to_string(),
        );
    }

    match registry_client::get_tags(
        &payload.image,
        payload.include.as_deref(),
        payload.exclude.as_deref(),
    )
    .await
    {
        Ok(json) => (StatusCode::OK, [("content-type", "application/json")], json),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            [("content-type", "text/plain")],
            format!("Error: {}", e),
        ),
    }
}
