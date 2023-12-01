use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

pub(crate) fn router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
}

async fn hello_world() -> impl IntoResponse {
    "Hello, world!"
}

async fn fake_error() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}
