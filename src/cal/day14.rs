use axum::{response::IntoResponse, routing::post, Json, Router};

pub(crate) fn router() -> Router {
    Router::new()
        .route("/14/unsafe", post(unsafefn))
        .route("/14/safe", post(safefn))
}

#[derive(serde::Deserialize)]
struct Content {
    content: String,
}

#[derive(askama::Template)]
#[template(path = "day14/index.html", escape = "none")]
struct UnsafeTemplate {
    content: String,
}

async fn unsafefn(Json(content): Json<Content>) -> impl IntoResponse {
    UnsafeTemplate {
        content: content.content,
    }
}

#[derive(askama::Template)]
#[template(path = "day14/index.html")]
struct SafeTemplate {
    content: String,
}

#[axum::debug_handler]
async fn safefn(Json(content): Json<Content>) -> impl IntoResponse {
    SafeTemplate {
        content: content.content,
    }
}
