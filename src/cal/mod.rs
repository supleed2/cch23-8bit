mod day00;

pub(crate) fn router() -> axum::Router {
    axum::Router::new()
        .nest("/", day00::router())
}
