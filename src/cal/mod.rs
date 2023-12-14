mod day00;
mod day01;
mod day04;
mod day06;
mod day07;
mod day08;
mod day11;
mod day12;
mod day13;
mod day14;

pub(crate) fn router(pool: sqlx::PgPool) -> axum::Router {
    axum::Router::new()
        .nest("/", day00::router())
        .nest("/", day01::router())
        .nest("/", day04::router())
        .nest("/", day06::router())
        .nest("/", day07::router())
        .nest("/", day08::router())
        .nest("/", day11::router())
        .nest("/", day12::router())
        .nest("/", day13::router(pool))
        .nest("/", day14::router())
}
