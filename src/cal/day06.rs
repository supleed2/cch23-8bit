use axum::{routing::post, Json, Router};

pub(crate) fn router() -> Router {
    Router::new().route("/6", post(count_elf))
}

#[derive(serde::Serialize)]
struct CountElf {
    elf: usize,
    elf_on_a_shelf: usize,
    shelf_with_no_elf: usize,
}

async fn count_elf(body: String) -> Json<CountElf> {
    let elf = body.matches("elf").count();
    let elf_on_a_shelf = body.matches("elf on a shelf").count();
    let shelf_with_no_elf = body.matches("shelf").count() - elf_on_a_shelf;
    Json(CountElf {
        elf,
        elf_on_a_shelf,
        shelf_with_no_elf,
    })
}
