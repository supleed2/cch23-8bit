use axum::{routing::post, Json, Router};

pub(crate) fn router() -> Router {
    Router::new().route("/6", post(count_elf))
}

#[derive(serde::Serialize)]
struct CountElf {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf: usize,
}

async fn count_elf(body: String) -> Json<CountElf> {
    let elf = body.matches("elf").count();
    let elf_on_a_shelf = "elf on a shelf".as_bytes();
    let elf_on_a_shelf = body
        .as_bytes()
        .windows(14)
        .filter(|&w| w == elf_on_a_shelf)
        .count();
    let shelf_with_no_elf = body.matches("shelf").count() - elf_on_a_shelf;
    Json(CountElf {
        elf,
        elf_on_a_shelf,
        shelf_with_no_elf,
    })
}
