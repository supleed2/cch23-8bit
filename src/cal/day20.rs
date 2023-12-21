use axum::{body::Bytes, http::StatusCode, response::IntoResponse, routing::post, Router};
use bytes::Buf;
use std::process::Command;

pub(crate) fn router() -> Router {
    Router::new()
        .route("/20/archive_files", post(archive_files))
        .route("/20/archive_files_size", post(archive_files_size))
        .route("/20/cookie", post(cookie))
}

async fn archive_files(body: Bytes) -> Result<impl IntoResponse, (StatusCode, String)> {
    Ok(tar::Archive::new(body.reader())
        .entries()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
        .count()
        .to_string())
}

async fn archive_files_size(body: Bytes) -> Result<impl IntoResponse, (StatusCode, String)> {
    Ok(tar::Archive::new(body.reader())
        .entries()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
        .filter_map(Result::ok)
        .map(|entry| entry.size())
        .sum::<u64>()
        .to_string())
}

async fn cookie(body: Bytes) -> Result<impl IntoResponse, (StatusCode, String)> {
    let dir = tempfile::tempdir_in(".")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut archive = tar::Archive::new(body.reader());
    archive
        .unpack(dir.path())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let log = Command::new("git")
        .args(["log", "christmas", "--format=%cn,%H"])
        .current_dir(dir.path())
        .output()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !log.status.success() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Git log failed, christmas branch may not exist".to_string(),
        ));
    }

    let output = String::from_utf8(log.stdout)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let commits = output
        .lines()
        .filter_map(|l| l.split_once(','))
        .collect::<Vec<_>>();

    for (author, hash) in commits {
        let Ok(Ok(files)) = Command::new("git")
            .args(["ls-tree", "-r", hash])
            .current_dir(dir.path())
            .output()
            .map(|o| String::from_utf8(o.stdout))
        else {
            continue;
        };

        let Some(file) = files.lines().find(|s| s.contains("santa.txt")) else {
            continue;
        };

        let Some(blob) = file.split_whitespace().nth(2) else {
            continue;
        };

        let Ok(santatxt) = Command::new("git")
            .args(["show", blob])
            .current_dir(dir.path())
            .output()
        else {
            continue;
        };

        if santatxt.status.success() {
            if let Ok(s) = String::from_utf8(santatxt.stdout) {
                if s.contains("COOKIE") {
                    return Ok(format!("{author} {hash}"));
                }
            }
        }
    }

    Err((StatusCode::BAD_REQUEST, "Commit not found".to_string()))
}
