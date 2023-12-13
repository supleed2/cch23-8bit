use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

#[derive(Clone)]
struct Day13State {
    pool: sqlx::PgPool,
}

pub(crate) fn router(pool: sqlx::PgPool) -> Router {
    Router::new()
        .route("/13/sql", get(sql))
        .route("/13/reset", post(reset))
        .route("/13/orders", post(orders))
        .route("/13/orders/total", get(total))
        .route("/13/orders/popular", get(popular))
        .with_state(Day13State { pool })
}

async fn sql(State(state): State<Day13State>) -> Result<impl IntoResponse, (StatusCode, String)> {
    Ok(sqlx::query!("select 20231213 as \"i32!\"")
        .fetch_one(&state.pool)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
        .i32
        .to_string())
}

async fn reset(State(state): State<Day13State>) -> Result<StatusCode, StatusCode> {
    sqlx::query!("drop table if exists orders")
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    sqlx::query!(
        "create table orders (
            id INT PRIMARY KEY,
            region_id INT,
            gift_name VARCHAR(50),
            quantity INT
        )"
    )
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

#[derive(serde::Deserialize)]
struct Order {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

async fn orders(
    State(state): State<Day13State>,
    Json(orders): Json<Vec<Order>>,
) -> Result<StatusCode, (StatusCode, String)> {
    for Order {
        id,
        region_id,
        gift_name,
        quantity,
    } in orders
    {
        sqlx::query!(
            "insert into orders values ($1, $2, $3, $4)",
            id,
            region_id,
            gift_name,
            quantity
        )
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }
    Ok(StatusCode::OK)
}

async fn total(State(state): State<Day13State>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let total = sqlx::query!("select sum(quantity) as \"i64!\" from orders")
        .fetch_one(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .i64;
    Ok(format!("{{\"total\":{total}}}"))
}

async fn popular(
    State(state): State<Day13State>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let gifts = sqlx::query!(
        "select sum(quantity) as \"sq!\", gift_name as \"gift_name!\"
        from orders group by gift_name order by \"sq!\" desc"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    match gifts.len() {
        0 => Ok("{\"popular\":null}".to_string()),
        1 => Ok(format!("{{\"popular\":\"{}\"}}", gifts[0].gift_name)),
        _ if gifts[0].sq == gifts[1].sq => Ok("{\"popular\":null}".to_string()),
        _ => Ok(format!("{{\"popular\":\"{}\"}}", gifts[0].gift_name)),
    }
}
