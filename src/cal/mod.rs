mod day00;
mod day01;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day18;
mod day19;
mod day20;
mod day21;

pub(crate) fn router(pool: sqlx::PgPool) -> axum::Router {
    axum::Router::new()
        .nest("/", day00::router())
        .nest("/", day01::router())
        .nest("/", day04::router())
        .nest("/", day05::router())
        .nest("/", day06::router())
        .nest("/", day07::router())
        .nest("/", day08::router())
        .nest("/", day11::router())
        .nest("/", day12::router())
        .nest("/", day13::router(pool.clone()))
        .nest("/", day14::router())
        .nest("/", day15::router())
        .nest("/", day18::router(pool))
        .nest("/", day19::router())
        .nest("/", day20::router())
        .nest("/", day21::router())
}
