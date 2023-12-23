use axum::response::IntoResponse;

use crate::router::Error;

pub async fn task_01() -> Result<impl IntoResponse, Error> {
    Ok("Hello, world!")
}

pub async fn task_02() -> Error {
    anyhow::Error::msg("Challenge -1 Task 2").into()
}
