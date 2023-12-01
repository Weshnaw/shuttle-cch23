use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};
use derive_more::{Display, Error};
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

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error))
        .route("/1/*x", get(day_01_cube_bits));

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
