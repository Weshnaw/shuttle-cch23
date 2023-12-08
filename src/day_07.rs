use axum::{http::HeaderMap, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::router::ResponseError;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct Decode {
    flour: u32,
    #[serde(rename = "chocolate chips")]
    choco: u32,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct Recipe {
    flour: u32,
    #[serde(rename = "chocolate chips")]
    choco: u32,
    sugar: u32,
    butter: u32,
    #[serde(rename = "baking powder")]
    baking: u32,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct BakeInput {
    recipe: Recipe,
    pantry: Recipe,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct BakeOutput {
    cookies: u32,
    pantry: Recipe,
}

pub async fn task_01(headers: HeaderMap) -> Result<impl IntoResponse, ResponseError> {
    let cookie = headers.get("Cookie").expect("failed to get cookie header");
    info!(?cookie);
    let recipe: Decode = serde_json::from_str(
        &String::from_utf8(
            rbase64::decode(&cookie.to_str().expect("failed to str cookie")["recipe=".len()..])
                .expect("unable to decode cookie"),
        )
        .expect("unable to parse String"),
    )
    .expect("failed to parse json");

    Ok(Json(Decode {
        flour: recipe.flour,
        choco: recipe.choco,
    }))
}

pub async fn task_02(headers: HeaderMap) -> Result<impl IntoResponse, ResponseError> {
    let cookie = headers.get("Cookie").expect("failed to get cookie header");
    debug!(?cookie);
    let base64 = &cookie.to_str().expect("failed to str cookie")["recipe=".len()..];
    debug!(?base64);
    let decoded = &String::from_utf8(rbase64::decode(base64).expect("unable to decode cookie"))
        .expect("unable to parse String");
    debug!(?decoded);
    let recipe: BakeInput = serde_json::from_str(decoded).expect("failed to parse json");
    info!(?decoded);

    let max_cookies = *[
        recipe.pantry.flour / recipe.recipe.flour,
        recipe.pantry.choco / recipe.recipe.choco,
        recipe.pantry.sugar / recipe.recipe.sugar,
        recipe.pantry.butter / recipe.recipe.butter,
        recipe.pantry.baking / recipe.recipe.baking,
    ]
    .iter()
    .min()
    .expect("min not found");

    Ok(Json(BakeOutput {
        cookies: max_cookies,
        pantry: Recipe {
            flour: recipe.pantry.flour - (recipe.recipe.flour * max_cookies),
            choco: recipe.pantry.choco - (recipe.recipe.choco * max_cookies),
            sugar: recipe.pantry.sugar - (recipe.recipe.sugar * max_cookies),
            butter: recipe.pantry.butter - (recipe.recipe.butter * max_cookies),
            baking: recipe.pantry.baking - (recipe.recipe.baking * max_cookies),
        },
    }))
}
#[cfg(test)]
mod tests {
    use crate::router::router;

    use super::*;

    use axum::http::StatusCode;
    use axum_test_helper::TestClient;

    #[tokio::test]
    async fn test_01() {
        let router = router();
        let client = TestClient::new(router);
        let res = client
            .get("/7/decode")
            .header(
                "Cookie",
                "recipe=eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==",
            )
            .send()
            .await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            res.json::<Decode>().await,
            Decode {
                flour: 100,
                choco: 20
            }
        );
    }

    #[tokio::test]
    async fn test_02() {
        let router = router();
        let client = TestClient::new(router);
        let res = client
            .get("/7/bake")
            .header(
                "Cookie",
                "recipe=eyJyZWNpcGUiOnsiZmxvdXIiOjk1LCJzdWdhciI6NTAsImJ1dHRlciI6MzAsImJha2luZyBwb3dkZXIiOjEwLCJjaG9jb2xhdGUgY2hpcHMiOjUwfSwicGFudHJ5Ijp7ImZsb3VyIjozODUsInN1Z2FyIjo1MDcsImJ1dHRlciI6MjEyMiwiYmFraW5nIHBvd2RlciI6ODY1LCJjaG9jb2xhdGUgY2hpcHMiOjQ1N319",
            )
            .send()
            .await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            res.json::<BakeOutput>().await,
            BakeOutput {
                cookies: 4,
                pantry: Recipe {
                    flour: 5,
                    choco: 257,
                    sugar: 307,
                    butter: 2002,
                    baking: 825
                }
            }
        );
    }
}
