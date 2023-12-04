use axum::{extract::Json, response::IntoResponse, routing::post, Router};

pub(crate) fn router() -> Router {
    Router::new()
        .route("/4/strength", post(strength))
        .route("/4/contest", post(contest))
}

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct Reindeer {
    name: String,
    strength: i32,
}

async fn strength(Json(reindeer): Json<Vec<Reindeer>>) -> impl IntoResponse {
    reindeer
        .into_iter()
        .map(|r| r.strength)
        .sum::<i32>()
        .to_string()
}

#[derive(serde::Deserialize)]
struct AdvReindeer {
    name: String,
    strength: i32,
    speed: f32,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies: i32,
}

#[derive(serde::Serialize)]
struct ContestResult {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

async fn contest(Json(reindeer): Json<Vec<AdvReindeer>>) -> Json<ContestResult> {
    let fastest = reindeer
        .iter()
        .max_by(|l, r| l.speed.total_cmp(&r.speed))
        .unwrap();
    let tallest = reindeer.iter().max_by_key(|r| r.height).unwrap();
    let magician = reindeer.iter().max_by_key(|r| r.snow_magic_power).unwrap();
    let consumer = reindeer.iter().max_by_key(|r| r.candies).unwrap();
    Json(ContestResult {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food
        ),
    })
}
