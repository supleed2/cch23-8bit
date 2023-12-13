mod cal;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: sqlx::PgPool) -> shuttle_axum::ShuttleAxum {
    Ok(axum::Router::new()
        .nest("/", cal::router(pool))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .into())
}
