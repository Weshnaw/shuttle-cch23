use std::{collections::HashMap, string::FromUtf8Error, sync::Arc, time::SystemTimeError};

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
use serde::{Deserialize, Serialize};
use shuttle_persist::{PersistError, PersistInstance};
use sqlx::PgPool;
use tokio::sync::{broadcast::Sender, RwLock};
use tower_http::services::ServeDir;
use tracing::warn;

use crate::{
    day_00, day_01, day_04, day_06, day_07, day_08, day_11, day_12, day_13, day_14, day_15, day_18,
    day_19, day_20, day_21, day_22,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Chat {
    pub user: Option<String>,
    pub message: String,
}

#[derive(Clone)]
pub struct State {
    pub client: reqwest::Client,
    pub persist: PersistInstance,
    pub pool: PgPool,
    pub views: Arc<RwLock<usize>>,
    pub rooms: Arc<RwLock<HashMap<usize, Arc<Sender<Chat>>>>>,
}

pub fn router(persist: PersistInstance, pool: PgPool) -> Router {
    let state = State {
        client: reqwest::Client::new(),
        persist,
        pool,
        views: Arc::new(RwLock::new(0)),
        rooms: Arc::new(RwLock::new(HashMap::new())),
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
        .route("/14/unsafe", post(day_14::task_01))
        .route("/14/safe", post(day_14::task_02))
        .route("/15/nice", post(day_15::task_01))
        .route("/15/game", post(day_15::task_02))
        .route("/18/reset", post(day_18::task_01_reset))
        .route("/18/orders", post(day_18::task_01_orders))
        .route("/18/regions", post(day_18::task_01_regions))
        .route("/18/regions/total", get(day_18::task_01_total))
        .route("/18/regions/top_list/:number", get(day_18::task_02))
        .route("/19/ws/ping", get(day_19::task_01))
        .route("/19/reset", post(day_19::task_02_reset))
        .route("/19/views", get(day_19::task_02_views))
        .route("/19/ws/room/:number/user/:name", get(day_19::task_02_room))
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
    #[from(ignore)]
    Base64DecodeError(#[error(not(source))] String),
    #[from(ignore)]
    HeadingNotFound(#[error(not(source))] String),
    #[from(ignore)]
    MaxNotFound(#[error(not(source))] String),
    ToStrError(ToStrError),
    RegexError(regex::Error),
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> axum::response::Response {
        warn!("{:?} Error occured", self);
        match self {
            Self::ChallengeNeg1 => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Challenge D-1 Task 2").into_response()
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "UNKNOWN ERROR").into_response(),
        }
    }
}
