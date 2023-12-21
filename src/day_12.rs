use std::{sync::Arc, time::SystemTime};

use axum::{
    extract::{self, Path, State},
    response::IntoResponse,
    Json,
};
use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};
use tracing::info;
use ulid::Ulid;
use uuid::Uuid;

use crate::router::{self, ResponseError};

pub async fn task_01_save(
    Path(id): Path<String>,
    State(state): State<Arc<router::State>>,
) -> Result<impl IntoResponse, ResponseError> {
    state
        .persist
        .save::<SystemTime>(&format!("day-12_{}", id), SystemTime::now())?;
    Ok(())
}

pub async fn task_01_load(
    Path(id): Path<String>,
    State(state): State<Arc<router::State>>,
) -> Result<impl IntoResponse, ResponseError> {
    let previous_time = state
        .persist
        .load(&format!("day-12_{}", id))
        .unwrap_or(SystemTime::now());

    let duration = SystemTime::now().duration_since(previous_time)?;
    Ok(duration.as_secs().to_string())
}

pub async fn task_02(
    extract::Json(ulids): extract::Json<Vec<String>>,
) -> Result<impl IntoResponse, ResponseError> {
    let result = ulids
        .into_iter()
        .filter_map(|str| Ulid::from_string(&str).ok())
        .map(|ulid| Uuid::from_u128(ulid.0))
        .rev()
        .collect::<Vec<Uuid>>();

    Ok(Json(result))
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct Response {
    #[serde(rename = "christmas eve")]
    eve: usize,
    #[serde(rename = "weekday")]
    day: usize,
    #[serde(rename = "in the future")]
    future: usize,
    #[serde(rename = "LSB is 1")]
    lsb: usize,
}

pub async fn task_03(
    Path(day): Path<u64>,
    extract::Json(ulids): extract::Json<Vec<String>>,
) -> Result<impl IntoResponse, ResponseError> {
    let result = ulids
        .into_iter()
        .filter_map(|str| Ulid::from_string(&str).ok())
        .collect::<Vec<Ulid>>();

    let now = SystemTime::now();

    let future = result
        .iter()
        .filter(|ulid| ulid.datetime().duration_since(now).is_ok())
        .count();

    let day = result
        .iter()
        .filter(|ulid| {
            let chrono_time: DateTime<Utc> = ulid.datetime().into();
            let weekday = chrono_time.weekday();

            use chrono::Weekday::*;
            match day {
                0 => Mon == weekday,
                1 => Tue == weekday,
                2 => Wed == weekday,
                3 => Thu == weekday,
                4 => Fri == weekday,
                5 => Sat == weekday,
                6 => Sun == weekday,
                _ => false,
            }
        })
        .count();

    let eve = result
        .iter()
        .filter(|ulid| {
            let chrono_time: DateTime<Utc> = ulid.datetime().into();

            let month = chrono_time.month();
            let day = chrono_time.day();

            month == 12 && day == 24
        })
        .count();

    let lsb = result
        .iter()
        .filter(|ulid| {
            let bits = ulid.0;

            let lsb = bits & 1;

            lsb == 1
        })
        .count();

    let res = Response {
        eve,
        day,
        future,
        lsb,
    };

    info!(?res);

    Ok(Json(res))
}
