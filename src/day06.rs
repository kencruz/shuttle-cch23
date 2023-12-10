use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ElfCount {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf: usize,
}

pub async fn elf_on_a_shelf(payload: String) -> Json<ElfCount> {
    let elves = &payload.matches("elf").count();
    let elves_on_shelf = &payload.matches("elf on a shelf").count();
    let shelves = &payload.matches("shelf").count();

    Json(ElfCount {
        elf: *elves,
        elf_on_a_shelf: *elves_on_shelf,
        shelf_with_no_elf: *shelves - *elves_on_shelf,
    })
}
