use axum::{extract::Multipart, response::IntoResponse, routing::post, Router};
use image::io::Reader;
use std::io::Cursor;
use tower_http::services::ServeDir;

pub(crate) fn router() -> Router {
    Router::new()
        .nest_service("/11/assets", ServeDir::new("src/assets/day11"))
        .route("/11/red_pixels", post(red_pixels))
}

async fn red_pixels(mut multipart: Multipart) -> impl IntoResponse {
    if let Ok(Some(field)) = multipart.next_field().await {
        Reader::new(Cursor::new(field.bytes().await.unwrap()))
            .with_guessed_format()
            .expect("Cursor io is infallible")
            .decode()
            .expect("Loading image should not fail")
            .into_rgb8()
            .pixels()
            .filter(|&p| u16::from(p.0[0]) > u16::from(p.0[1]) + u16::from(p.0[2]))
            .count()
            .to_string()
    } else {
        "Did not get a file".to_string()
    }
}
