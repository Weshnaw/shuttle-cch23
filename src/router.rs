use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use derive_more::{Display, Error};
use tracing::warn;

use crate::{day_00, day_01, day_04, day_06, day_07};

pub fn router() -> Router {
    Router::new()
        .route("/", get(day_00::task_01))
        .route("/-1/error", get(day_00::task_02))
        .route("/1/*x", get(day_01::task_00))
        .route("/4/strength", post(day_04::task_01))
        .route("/4/contest", post(day_04::task_02))
        .route("/6", post(day_06::task_00))
        .route("/7/decode", get(day_07::task_01))
        .route("/7/bake", get(day_07::task_02))
}

#[derive(Error, Display, Debug)]
pub enum ResponseError {
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
