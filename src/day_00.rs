use axum::response::IntoResponse;
use tracing::debug;

use crate::router::ResponseError;

pub async fn task_01() -> Result<impl IntoResponse, ResponseError> {
    Ok("Hello, world!")
}

pub async fn task_02() -> ResponseError {
    debug!("ChallengeNeg1 Error always sent at this path");
    ResponseError::ChallengeNeg1
}
