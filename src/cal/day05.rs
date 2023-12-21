use axum::{
    extract::Query,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};

pub(crate) fn router() -> Router {
    Router::new().route("/5", post(five))
}

#[derive(serde::Deserialize)]
struct Pagination {
    offset: Option<usize>,
    limit: Option<usize>,
    split: Option<usize>,
}

async fn five(Query(pagination): Query<Pagination>, Json(names): Json<Vec<String>>) -> Response {
    let offset = pagination.offset.unwrap_or(0);
    let limit = pagination.limit.unwrap_or(names.len());
    let names = names
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect::<Vec<_>>();
    if let Some(split) = pagination.split {
        let names = names.chunks(split).map(|a| a.to_vec()).collect::<Vec<_>>();
        Json(names).into_response()
    } else {
        Json(names).into_response()
    }
}
