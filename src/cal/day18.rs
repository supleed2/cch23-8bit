use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

#[derive(Clone)]
struct Day18State {
    pool: sqlx::PgPool,
}

pub(crate) fn router(pool: sqlx::PgPool) -> Router {
    Router::new()
        .route("/18/reset", post(reset))
        .route("/18/orders", post(orders))
        .route("/18/regions", post(regions))
        .route("/18/regions/total", get(total))
        .route("/18/regions/top_list/:count", get(top_list))
        .with_state(Day18State { pool })
}

async fn reset(State(state): State<Day18State>) -> Result<StatusCode, StatusCode> {
    sqlx::query!("drop table if exists regions")
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    sqlx::query!("drop table if exists orders")
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    sqlx::query!(
        "create table regions (
            id INT PRIMARY KEY,
            name VARCHAR(50)
        )"
    )
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
    State(state): State<Day18State>,
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

#[derive(serde::Deserialize)]
struct Region {
    id: i32,
    name: String,
}

async fn regions(
    State(state): State<Day18State>,
    Json(regions): Json<Vec<Region>>,
) -> Result<StatusCode, (StatusCode, String)> {
    for Region { id, name } in regions {
        sqlx::query!("insert into regions values ($1, $2)", id, name)
            .execute(&state.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }
    Ok(StatusCode::OK)
}

#[derive(serde::Serialize)]
struct Total {
    region: String,
    total: i64,
}

async fn total(State(state): State<Day18State>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let totals = sqlx::query_as!(
        Total,
        "select name as \"region!\", sum(quantity) as \"total!\"
        from orders join regions on orders.region_id = regions.id
        group by name order by name"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(totals))
}

#[derive(serde::Serialize)]
struct TopGifts {
    region: String,
    top_gifts: Vec<String>,
}

async fn top_list(
    Path(limit): Path<i64>,
    State(state): State<Day18State>,
) -> Result<Json<Vec<TopGifts>>, (StatusCode, String)> {
    let mut top_list = vec![];

    let regions = sqlx::query_as!(
        Region,
        "select id, name as \"name!\" from regions order by name"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    for region in regions {
        let top_gifts = sqlx::query!(
            "select gift_name as \"gift_name!\" from orders where region_id = $1 group by gift_name order by sum(quantity) desc limit $2",
            region.id,
            limit
        )
        .fetch_all(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .into_iter()
        .map(|r| r.gift_name)
        .collect();

        top_list.push(TopGifts {
            region: region.name,
            top_gifts,
        });
    }

    Ok(Json(top_list))
}
