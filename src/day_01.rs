use axum::{extract::Path, response::IntoResponse};
use tracing::info;

use crate::router::ResponseError;

pub async fn task_00(Path(x): Path<String>) -> Result<impl IntoResponse, ResponseError> {
    info!(?x);

    let sum = x
        .split('/')
        .map(|x| x.parse::<i32>().unwrap_or(0))
        .fold(0, |acc, x| acc ^ x);

    let result = sum.pow(3);

    Ok(result.to_string())
}
