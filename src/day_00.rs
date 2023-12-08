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
#[cfg(test)]
mod tests {
    use crate::router::router;

    use axum::http::StatusCode;
    use axum_test_helper::TestClient;

    #[tokio::test]
    async fn test_01() {
        let router = router();
        let client = TestClient::new(router);
        let res = client.get("/").send().await;
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_02() {
        let router = router();
        let client = TestClient::new(router);
        let res = client.get("/-1/error").send().await;
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
