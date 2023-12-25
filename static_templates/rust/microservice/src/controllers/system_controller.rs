use axum::response::IntoResponse;
use axum::Json;

pub async fn alive() -> impl IntoResponse {
    Json(AliveResponse::new()).into_response()
}

#[derive(Debug, Clone, serde::Serialize)]
enum AliveResponse {
    Alive,
}

impl AliveResponse {
    fn new() -> Self {
        Self::Alive
    }
}
