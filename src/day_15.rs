use axum::response::IntoResponse;

use crate::router::ResponseError;

pub async fn task_01() -> Result<impl IntoResponse, ResponseError> {
    Ok("todo")
}

pub async fn task_02() -> Result<impl IntoResponse, ResponseError> {
    Ok("todo")
}
