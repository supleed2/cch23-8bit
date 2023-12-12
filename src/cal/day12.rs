use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Datelike, Utc};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::SystemTime,
};

struct Day12State {
    times: HashMap<String, SystemTime>,
}

pub(crate) fn router() -> Router {
    Router::new()
        .route("/12/save/:string", post(save))
        .route("/12/load/:string", get(load))
        .route("/12/ulids", post(ulids))
        .route("/12/ulids/:weekday", post(ulids_weekday))
        .with_state(Arc::new(Mutex::new(Day12State {
            times: HashMap::new(),
        })))
}

async fn save(State(state): State<Arc<Mutex<Day12State>>>, Path(string): Path<String>) {
    state
        .lock()
        .expect("Should not error unless another thread panics")
        .times
        .insert(string, SystemTime::now());
}

async fn load(
    State(state): State<Arc<Mutex<Day12State>>>,
    Path(string): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    Ok(state
        .lock()
        .expect("Should not error unless another thread panics")
        .times
        .get(&string)
        .ok_or((
            StatusCode::NOT_FOUND,
            "String not saved previously".to_string(),
        ))?
        .elapsed()
        .expect("Time should not go backwards")
        .as_secs()
        .to_string())
}

async fn ulids(Json(ulids): Json<Vec<ulid::Ulid>>) -> Json<Vec<uuid::Uuid>> {
    Json(ulids.into_iter().map(Into::into).rev().collect())
}

#[derive(serde::Serialize)]
struct UlidsWeekday {
    #[serde(rename = "christmas eve")]
    christmas_eve: usize,
    weekday: usize,
    #[serde(rename = "in the future")]
    in_the_future: usize,
    #[serde(rename = "LSB is 1")]
    lsb_is_1: usize,
}

#[axum::debug_handler]
async fn ulids_weekday(
    Path(weekday): Path<u32>,
    Json(ulids): Json<Vec<ulid::Ulid>>,
) -> Json<UlidsWeekday> {
    let lsb_is_1 = ulids.iter().filter(|u| u.random() & 1 == 1).count();
    let ulids: Vec<DateTime<Utc>> = ulids.into_iter().map(|u| u.datetime().into()).collect();
    let christmas_eve = ulids
        .iter()
        .filter(|u| u.month() == 12 && u.day() == 24)
        .count();
    let weekday = ulids
        .iter()
        .filter(|u| u.weekday().num_days_from_monday() == weekday)
        .count();
    let now = Utc::now();
    let in_the_future = ulids.iter().filter(|&u| u > &now).count();
    Json(UlidsWeekday {
        christmas_eve,
        weekday,
        in_the_future,
        lsb_is_1,
    })
}
