use axum::{
    extract::{self, Path},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

async fn hello_world() -> Result<impl IntoResponse, ResponseError> {
    Ok("Hello, world!")
}

async fn error() -> ResponseError {
    debug!("ChallengeNeg1 Error always sent at this path");
    ResponseError::ChallengeNeg1
}

async fn day_01_cube_bits(Path(x): Path<String>) -> Result<impl IntoResponse, ResponseError> {
    info!(?x);

    let sum = x
        .split('/')
        .map(|x| x.parse::<i32>().unwrap_or(0))
        .fold(0, |acc, x| acc ^ x);

    let result = sum.pow(3);

    Ok(result.to_string())
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Reindeer {
    name: String,
    #[serde(default)]
    strength: i32,
    speed: f32,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies: i32,
}

async fn day_04_strength(
    extract::Json(payload): extract::Json<Vec<Reindeer>>,
) -> Result<impl IntoResponse, ResponseError> {
    info!(?payload);

    let result: i32 = payload.iter().map(|rein| rein.strength).sum();

    Ok(result.to_string())
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct Contest {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

impl From<Vec<Reindeer>> for Contest {
    fn from(value: Vec<Reindeer>) -> Self {
        let fastest = value
            .iter()
            .max_by(|rein_x, rein_y| rein_x.speed.total_cmp(&rein_y.speed))
            .unwrap();
        let tallest = value.iter().max_by_key(|rein| rein.height).unwrap();
        let magician = value
            .iter()
            .max_by_key(|rein| rein.snow_magic_power)
            .unwrap();
        let consumer = value.iter().max_by_key(|rein| rein.candies).unwrap();

        Contest {
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
        }
    }
}

async fn day_04_contest(
    extract::Json(payload): extract::Json<Vec<Reindeer>>,
) -> Result<impl IntoResponse, ResponseError> {
    info!(?payload);

    let result: Contest = payload.into();

    Ok(Json(result))
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct ElfCount {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_shelfs: usize,
    #[serde(rename = "shelf with no elf on it")]
    no_elf_shelfs: usize,
}

async fn day_06_elf_count(body: String) -> Result<impl IntoResponse, ResponseError> {
    let elf = body.to_lowercase().matches("elf").count();
    let elf_shelfs = body.to_lowercase().matches("elf on a shelf").count();
    let no_elf_shelfs = body.to_lowercase().matches("shelf").count() - elf_shelfs;
    Ok(Json(ElfCount {
        elf,
        elf_shelfs,
        no_elf_shelfs,
    }))
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct Decode {
    flour: u32,
    #[serde(rename = "chocolate chips")]
    choco: u32,
}

async fn day_07_decode(headers: HeaderMap) -> Result<impl IntoResponse, ResponseError> {
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

async fn day_07_bake(headers: HeaderMap) -> Result<impl IntoResponse, ResponseError> {
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

fn router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error))
        .route("/1/*x", get(day_01_cube_bits))
        .route("/4/strength", post(day_04_strength))
        .route("/4/contest", post(day_04_contest))
        .route("/6", post(day_06_elf_count))
        .route("/7/decode", get(day_07_decode))
        .route("/7/bake", get(day_07_bake))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(router().into())
}

#[derive(Error, Display, Debug)]
enum ResponseError {
    #[allow(dead_code)]
    UnkownError(#[error(not(source))] String),
    ChallengeNeg1,
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> axum::response::Response {
        warn!("{:?} Error occured", self);
        match self {
            Self::UnkownError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
            Self::ChallengeNeg1 => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Challenge D-1 Task 2").into_response()
            }
            #[allow(unreachable_patterns)]
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "UNKOWN ERROR").into_response(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test_helper::TestClient;

    #[tokio::test]
    async fn test_day_neg_one_task_one() {
        let router = router();
        let client = TestClient::new(router);
        let res = client.get("/").send().await;
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_day_neg_one_task_two() {
        let router = router();
        let client = TestClient::new(router);
        let res = client.get("/-1/error").send().await;
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[rstest::rstest]
    #[case("4/8", "1728")]
    #[case("10", "1000")]
    #[case("4/5/8/10", "27")]
    #[tokio::test]
    async fn test_day_one(#[case] input: &str, #[case] expected: &str) {
        let router = router();
        let client = TestClient::new(router);
        let res = client.get(&format!("/1/{}", input)).send().await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await, expected);
    }

    #[tokio::test]
    async fn test_day_four_task_one() {
        let router = router();
        let client = TestClient::new(router);
        let res = client
            .post("/4/strength")
            .json(&[
                Reindeer {
                    name: "Dasher".to_string(),
                    strength: 5,
                    speed: 0f32,
                    height: 0,
                    antler_width: 0,
                    snow_magic_power: 0,
                    favorite_food: "Unknown".to_string(),
                    candies: 0,
                },
                Reindeer {
                    name: "Dancer".to_string(),
                    strength: 6,
                    speed: 0f32,
                    height: 0,
                    antler_width: 0,
                    snow_magic_power: 0,
                    favorite_food: "Unknown".to_string(),
                    candies: 0,
                },
                Reindeer {
                    name: "Prancer".to_string(),
                    strength: 4,
                    speed: 0f32,
                    height: 0,
                    antler_width: 0,
                    snow_magic_power: 0,
                    favorite_food: "Unknown".to_string(),
                    candies: 0,
                },
                Reindeer {
                    name: "Vixen".to_string(),
                    strength: 7,
                    speed: 0f32,
                    height: 0,
                    antler_width: 0,
                    snow_magic_power: 0,
                    favorite_food: "Unknown".to_string(),
                    candies: 0,
                },
            ])
            .send()
            .await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await, "22");
    }

    #[tokio::test]
    async fn test_day_four_task_two() {
        let router = router();
        let client = TestClient::new(router);
        let res = client
            .post("/4/contest")
            .json(&[
                Reindeer {
                    name: "Dasher".to_string(),
                    strength: 5,
                    speed: 50.4,
                    height: 80,
                    antler_width: 36,
                    snow_magic_power: 9001,
                    favorite_food: "hay".to_string(),
                    candies: 2,
                },
                Reindeer {
                    name: "Dancer".to_string(),
                    strength: 6,
                    speed: 48.2,
                    height: 65,
                    antler_width: 37,
                    snow_magic_power: 4004,
                    favorite_food: "grass".to_string(),
                    candies: 6,
                },
            ])
            .send()
            .await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            res.json::<Contest>().await,
            Contest {
                fastest: "Speeding past the finish line with a strength of 5 is Dasher".to_string(),
                tallest: "Dasher is standing tall with his 36 cm wide antlers".to_string(),
                magician: "Dasher could blast you away with a snow magic power of 9001".to_string(),
                consumer: "Dancer ate lots of candies, but also some grass".to_string()
            }
        );
    }

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
    async fn test_day_six(#[case] input: &str, #[case] expected: ElfCount) {
        let router = router();
        let client = TestClient::new(router);
        let res = client.post("/6").body(input.to_string()).send().await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.json::<ElfCount>().await, expected);
    }

    #[tokio::test]
    async fn test_day_seven_task_one() {
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
    async fn test_day_seven_task_two() {
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
