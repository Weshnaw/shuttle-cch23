use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::router::ResponseError;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct ElfCount {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_shelfs: usize,
    #[serde(rename = "shelf with no elf on it")]
    no_elf_shelfs: usize,
}

pub async fn both_tasks(body: String) -> Result<impl IntoResponse, ResponseError> {
    let elf = body.to_lowercase().matches("elf").count();
    let elf_shelfs = body.to_lowercase().matches("elf on a shelf").count();
    let no_elf_shelfs = body.to_lowercase().matches("shelf").count() - elf_shelfs;
    Ok(Json(ElfCount {
        elf,
        elf_shelfs,
        no_elf_shelfs,
    }))
}

#[cfg(test)]
mod tests {
    use crate::router::router;

    use super::*;

    use axum::http::StatusCode;
    use axum_test_helper::TestClient;

    #[rstest::rstest]
    #[case(
        "The mischievous elf peeked out from behind the toy workshop,
         and another elf joined in the festive dance.
         Look, there is also an elf on that shelf!",
        ElfCount { elf: 4, elf_shelfs: 0, no_elf_shelfs: 1}
    )]
    #[case(
        "there is an elf on a shelf on an elf.
         there is also another shelf in Belfast.",
        ElfCount { elf: 5, elf_shelfs: 1, no_elf_shelfs: 1}
    )]
    #[tokio::test]
    async fn test_both_tasks(#[case] input: &str, #[case] expected: ElfCount) {
        let router = router();
        let client = TestClient::new(router);
        let res = client.post("/6").body(input.to_string()).send().await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.json::<ElfCount>().await, expected);
    }
}
