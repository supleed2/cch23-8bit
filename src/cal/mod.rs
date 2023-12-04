mod day00;
mod day01;
mod day04;

pub(crate) fn router() -> axum::Router {
    axum::Router::new()
        .nest("/", day00::router())
        .nest("/", day01::router())
        .nest("/", day04::router())
}
