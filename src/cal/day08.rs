use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};

pub(crate) fn router() -> Router {
    Router::new()
        .route("/8/weight/:pokedex_number", get(weight))
        .route("/8/drop/:pokedex_number", get(drop))
}

#[derive(serde::Deserialize)]
struct PokeWeight {
    weight: f32,
}

async fn weight(
    Path(pokedex_number): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    pokemon_weight(pokedex_number).await.map(|f| f.to_string())
}

async fn drop(Path(pokedex_number): Path<i32>) -> Result<impl IntoResponse, (StatusCode, String)> {
    Ok((14.017_845f32 * pokemon_weight(pokedex_number).await?).to_string())
}

async fn pokemon_weight(pokedex_number: i32) -> Result<f32, (StatusCode, String)> {
    Ok(reqwest::get(format!(
        "https://pokeapi.co/api/v2/pokemon/{pokedex_number}"
    ))
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, format!("{e:?}")))?
    .json::<PokeWeight>()
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, format!("{e:?}")))?
    .weight
        / 10f32)
}
