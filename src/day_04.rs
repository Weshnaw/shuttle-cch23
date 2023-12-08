use axum::{
    extract::{self},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::router::ResponseError;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Reindeer {
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

pub async fn task_01(
    extract::Json(payload): extract::Json<Vec<Reindeer>>,
) -> Result<impl IntoResponse, ResponseError> {
    info!(?payload);

    let result: i32 = payload.iter().map(|rein| rein.strength).sum();

    Ok(result.to_string())
}

pub async fn task_02(
    extract::Json(payload): extract::Json<Vec<Reindeer>>,
) -> Result<impl IntoResponse, ResponseError> {
    info!(?payload);

    Ok(Json(Contest::from(payload)))
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
    async fn test_02() {
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
}
