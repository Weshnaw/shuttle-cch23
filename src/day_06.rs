use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::router::Error;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct ElfCount {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_shelfs: usize,
    #[serde(rename = "shelf with no elf on it")]
    no_elf_shelfs: usize,
}

const ELF_ON_A_SHELF: &str = "elf on a shelf";
pub async fn task_00(body: String) -> Result<impl IntoResponse, Error> {
    // let body = body.to_lowercase();
    info!(?body);
    let elf = body.matches("elf").count();
    let elf_shelfs = body
        .chars()
        .map_windows(|w: &[_; ELF_ON_A_SHELF.len()]| {
            if String::from_iter(w) == *ELF_ON_A_SHELF {
                1
            } else {
                0
            }
        })
        .sum();
    let no_elf_shelfs = body.matches("shelf").count() - elf_shelfs;
    let elf_count = ElfCount {
        elf,
        elf_shelfs,
        no_elf_shelfs,
    };
    info!(?elf_count);
    Ok(Json(elf_count))
}
