use std::{string::FromUtf8Error, time::SystemTimeError};

use axum::{
    extract::multipart::MultipartError,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use derive_more::{Display, Error, From};
use image::ImageError;
use reqwest::header::ToStrError;
use shuttle_persist::{PersistError, PersistInstance};
use sqlx::PgPool;
use tower_http::services::ServeDir;
use tracing::warn;

use crate::{
    day_00, day_01, day_04, day_06, day_07, day_08, day_11, day_12, day_13, day_14, day_15, day_18,
    day_19, day_20, day_21, day_22,
};

#[derive(Clone)]
pub struct State {
    pub client: reqwest::Client,
    pub persist: PersistInstance,
    pub pool: PgPool,
}

pub fn router(persist: PersistInstance, pool: PgPool) -> Router {
    let state = State {
        client: reqwest::Client::new(),
        persist,
        pool,
    };

    Router::new()
        .route("/", get(day_00::task_01))
        .route("/-1/error", get(day_00::task_02))
        .route("/1/*x", get(day_01::task_00))
        .route("/4/strength", post(day_04::task_01))
        .route("/4/contest", post(day_04::task_02))
        .route("/6", post(day_06::task_00))
        .route("/7/decode", get(day_07::task_01))
        .route("/7/bake", get(day_07::task_02))
        .route("/8/weight/:number", get(day_08::task_01))
        .route("/8/drop/:number", get(day_08::task_02))
        .nest_service("/11/assets/", ServeDir::new("assets"))
        .route("/11/red_pixels", post(day_11::task_02))
        .route("/12/save/:id", post(day_12::task_01_save))
        .route("/12/load/:id", get(day_12::task_01_load))
        .route("/12/ulids", post(day_12::task_02))
        .route("/12/ulids/:day", post(day_12::task_03))
        .route("/13/sql", get(day_13::task_01))
        .route("/13/reset", post(day_13::task_02_reset))
        .route("/13/orders", post(day_13::task_02_orders))
        .route("/13/orders/total", get(day_13::task_02_total))
        .route("/13/orders/popular", get(day_13::task_03_popular))
        .route("/14/1", get(day_14::task_01))
        .route("/14/2", get(day_14::task_02))
        .route("/15/1", get(day_15::task_01))
        .route("/15/2", get(day_15::task_02))
        .route("/18/1", get(day_18::task_01))
        .route("/18/2", get(day_18::task_02))
        .route("/19/1", get(day_19::task_01))
        .route("/19/2", get(day_19::task_02))
        .route("/20/1", get(day_20::task_01))
        .route("/20/2", get(day_20::task_02))
        .route("/21/1", get(day_21::task_01))
        .route("/21/2", get(day_21::task_02))
        .route("/22/1", get(day_22::task_01))
        .route("/22/2", get(day_22::task_02))
        .with_state(state)
}

#[derive(Error, Display, Debug, From)]
pub enum ResponseError {
    #[allow(dead_code)]
    UnkownError(#[error(not(source))] String),
    ChallengeNeg1,
    SqlError(sqlx::Error),
    PersistError(PersistError),
    SystemTimeError(SystemTimeError),
    MultiPartError(MultipartError),
    ImageError(ImageError),
    IoError(std::io::Error),
    ReqwestError(reqwest::Error),
    JsonError(serde_json::Error),
    Utf8StringError(FromUtf8Error),
    Base64DecodeError,
    #[from(ignore)]
    HeadingNotFound(#[error(not(source))] String),
    #[from(ignore)]
    MaxNotFound(#[error(not(source))] String),
    ToStrError(ToStrError),
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
