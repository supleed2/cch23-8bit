use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};

pub(crate) fn router() -> Router {
    Router::new().route("/1/*ids", get(cube_bits))
}

async fn cube_bits(Path(ids): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    let res = ids
        .split('/')
        .map(|id| id.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into_iter()
        .fold(0i32, |acc, id| acc ^ id)
        .pow(3)
        .to_string();
    Ok(res)
}
