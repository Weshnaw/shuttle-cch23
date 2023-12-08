use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use derive_more::{Display, Error};
use tracing::warn;

use crate::{day_four, day_neg_one, day_one, day_seven, day_six};

pub fn router() -> Router {
    Router::new()
        .route("/", get(day_neg_one::task_one))
        .route("/-1/error", get(day_neg_one::task_two))
        .route("/1/*x", get(day_one::both_tasks))
        .route("/4/strength", post(day_four::task_one))
        .route("/4/contest", post(day_four::task_two))
        .route("/6", post(day_six::both_tasks))
        .route("/7/decode", get(day_seven::task_one))
        .route("/7/bake", get(day_seven::task_two))
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
