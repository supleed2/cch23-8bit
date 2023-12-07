use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use axum_extra::{headers::Cookie, TypedHeader};
use base64::{engine::general_purpose, Engine as _};
use std::collections::HashMap;

pub(crate) fn router() -> Router {
    Router::new()
        .route("/7/decode", get(decode))
        .route("/7/bake", get(bake))
}

async fn decode(
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let recipe = cookie
        .get("recipe")
        .ok_or((StatusCode::BAD_REQUEST, "recipe cookie missing".to_string()))?;
    let decoded = general_purpose::STANDARD
        .decode(recipe)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("{e:?}")))?;
    let json =
        String::from_utf8(decoded).map_err(|e| (StatusCode::BAD_REQUEST, format!("{e:?}")))?;
    Ok(json)
}

#[derive(serde::Deserialize)]
struct Bake {
    recipe: HashMap<String, i32>,
    pantry: HashMap<String, i32>,
}

#[derive(serde::Serialize)]
struct Cookies {
    cookies: i32,
    pantry: HashMap<String, i32>,
}

async fn bake(
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<Json<Cookies>, (StatusCode, String)> {
    let recipe = cookie
        .get("recipe")
        .ok_or((StatusCode::BAD_REQUEST, "recipe cookie missing".to_string()))?;
    let decoded = general_purpose::STANDARD
        .decode(recipe)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("{e:?}")))?;
    let Bake { recipe, mut pantry } = serde_json::from_slice(&decoded)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("{e:?}")))?;
    if let Some(cookies) = recipe
        .iter()
        .map(|(i, a)| pantry.get(i).map(|p| p / a))
        .collect::<Option<Vec<_>>>()
        .and_then(|v| v.into_iter().min())
    {
        for (i, a) in recipe {
            *pantry.get_mut(&i).unwrap() -= a * cookies;
        }
        Ok(Json(Cookies { cookies, pantry }))
    } else {
        Ok(Json(Cookies { cookies: 0, pantry }))
    }
}
