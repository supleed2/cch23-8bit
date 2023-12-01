mod cal;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(axum::Router::new().nest("/", cal::router()).into())
}
