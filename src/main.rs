use axum::{
    extract::{self, Path},
    http::StatusCode,
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
        .split("/")
        .map(|x| x.parse::<i32>().unwrap_or(0))
        .fold(0, |acc, x| acc ^ x);

    let result = sum.pow(3);

    Ok(result.to_string())
}

#[derive(Deserialize, Debug)]
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

#[derive(Serialize, Debug)]
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

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error))
        .route("/1/*x", get(day_01_cube_bits))
        .route("/4/strength", post(day_04_strength))
        .route("/4/contest", post(day_04_contest));

    Ok(router.into())
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
