use axum::response::IntoResponse;
use tracing::debug;

use crate::router::ResponseError;

pub async fn task_one() -> Result<impl IntoResponse, ResponseError> {
    Ok("todo")
}

pub async fn task_two() -> ResponseError {
    Ok("todo")
}

#[cfg(test)]
mod tests {
    use crate::router::router;

    use axum::http::StatusCode;
    use axum_test_helper::TestClient;

    #[tokio::test]
    async fn test_task_one() {
        let router = router();
        let client = TestClient::new(router);
        let res = client.get("/").send().await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await, "");
    }

    #[tokio::test]
    async fn test_task_two() {
        let router = router();
        let client = TestClient::new(router);
        let res = client.get("/").send().await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await, "");
    }
}
