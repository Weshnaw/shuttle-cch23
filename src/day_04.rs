use axum::{
    extract::{self},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::router::Error;

#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
#[serde(default)]
pub struct Reindeer {
    name: String,
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
            .cloned()
            .unwrap_or_default();
        let tallest = value
            .iter()
            .max_by_key(|rein| rein.height)
            .cloned()
            .unwrap_or_default();
        let magician = value
            .iter()
            .max_by_key(|rein| rein.snow_magic_power)
            .cloned()
            .unwrap_or_default();
        let consumer = value
            .iter()
            .max_by_key(|rein| rein.candies)
            .cloned()
            .unwrap_or_default();

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
) -> Result<impl IntoResponse, Error> {
    info!(?payload);

    let result: i32 = payload.iter().map(|rein| rein.strength).sum();

    Ok(result.to_string())
}

pub async fn task_02(
    extract::Json(payload): extract::Json<Vec<Reindeer>>,
) -> Result<impl IntoResponse, Error> {
    info!(?payload);

    Ok(Json(Contest::from(payload)))
}
