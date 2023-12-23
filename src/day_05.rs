use axum::{extract::Query, response::IntoResponse, Json};
use serde::Deserialize;

use crate::router::Error;

#[derive(Deserialize, Debug)]
pub struct Pagination {
    offset: Option<usize>,
    limit: Option<usize>,
    split: Option<usize>,
}
pub async fn task_00(
    pagination: Query<Pagination>,
    Json(payload): Json<Vec<String>>,
) -> Result<impl IntoResponse, Error> {
    let size = payload.len();
    let result = payload
        .into_iter()
        .skip(pagination.offset.unwrap_or_default())
        .take(pagination.limit.unwrap_or(size))
        .collect::<Vec<_>>();

    if let Some(split) = pagination.split {
        let result = result.chunks(split).collect::<Vec<_>>();
        Ok(Json(result).into_response())
    } else {
        Ok(Json(result).into_response())
    }
}
