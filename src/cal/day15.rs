use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};

pub(crate) fn router() -> Router {
    Router::new()
        .route("/15/nice", post(nice))
        .route("/15/game", post(game))
}

#[derive(serde::Deserialize)]
struct Nice {
    input: String,
}

async fn nice(Json(Nice { input }): Json<Nice>) -> impl IntoResponse {
    let vowels = input
        .chars()
        .filter(|&c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' || c == 'y')
        .count();
    let repeat = input
        .as_bytes()
        .windows(2)
        .any(|c| c[0].is_ascii_alphabetic() && c[0] == c[1]);
    let substrs = input.contains("ab")
        || input.contains("cd")
        || input.contains("pq")
        || input.contains("xy");
    if vowels > 2 && repeat && !substrs {
        (StatusCode::OK, "{\"result\":\"nice\"}".to_string())
    } else {
        (
            StatusCode::BAD_REQUEST,
            "{\"result\":\"naughty\"}".to_string(),
        )
    }
}

#[derive(serde::Serialize)]
struct Game {
    result: String,
    reason: String,
}

async fn game(Json(Nice { input }): Json<Nice>) -> (StatusCode, Json<Game>) {
    let code;
    let result;
    let reason;
    let r1 = input.len() >= 8;
    let r2 = input.chars().any(|c| c.is_ascii_uppercase())
        && input.chars().any(|c| c.is_ascii_lowercase())
        && input.chars().any(|c| c.is_ascii_digit());
    let r3 = input.chars().filter(char::is_ascii_digit).count() >= 5;
    let r4 = regex::Regex::new(r"\d+")
        .expect("Regex should be valid")
        .captures_iter(&input)
        .map(|c| c.extract::<0>().0)
        .map(|s| s.parse::<i32>().expect("All matches are only digits"))
        .sum::<i32>()
        == 2023;
    let r5 = regex::Regex::new(r".*j.*o.*y.*")
        .expect("Regex should be valid")
        .is_match(&input)
        && input.chars().filter(|&c| c == 'j').count() == 1
        && input.chars().filter(|&c| c == 'o').count() == 1
        && input.chars().filter(|&c| c == 'y').count() == 1;
    let r6 = input.as_bytes().windows(3).any(|c| {
        c[0] != c[1] && c[0] == c[2] && c[0].is_ascii_alphabetic() && c[1].is_ascii_alphabetic()
    });
    let r7 = input
        .chars()
        .any(|c| ('\u{2980}'..='\u{2BFF}').contains(&c));
    let r8 = regex::Regex::new(r"[\p{Emoji}--\p{Ascii}]")
        .expect("Regex should be valid")
        .is_match(&input);
    let r9 = sha256::digest(input.clone()).ends_with('a');
    if !r1 {
        code = StatusCode::BAD_REQUEST;
        result = "naughty".to_string();
        reason = "8 chars".to_string();
    } else if !r2 {
        code = StatusCode::BAD_REQUEST;
        result = "naughty".to_string();
        reason = "more types of chars".to_string();
    } else if !r3 {
        code = StatusCode::BAD_REQUEST;
        result = "naughty".to_string();
        reason = "55555".to_string();
    } else if !r4 {
        code = StatusCode::BAD_REQUEST;
        result = "naughty".to_string();
        reason = "math is hard".to_string();
    } else if !r5 {
        code = StatusCode::NOT_ACCEPTABLE;
        result = "naughty".to_string();
        reason = "not joyful enough".to_string();
    } else if !r6 {
        code = StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS;
        result = "naughty".to_string();
        reason = "illegal: no sandwich".to_string();
    } else if !r7 {
        code = StatusCode::RANGE_NOT_SATISFIABLE;
        result = "naughty".to_string();
        reason = "outranged".to_string();
    } else if !r8 {
        code = StatusCode::UPGRADE_REQUIRED;
        result = "naughty".to_string();
        reason = "😳".to_string();
    } else if !r9 {
        code = StatusCode::IM_A_TEAPOT;
        result = "naughty".to_string();
        reason = "not a coffee brewer".to_string();
    } else {
        code = StatusCode::OK;
        result = "nice".to_string();
        reason = "that's a nice password".to_string();
    }
    (code, Json(Game { result, reason }))
}
