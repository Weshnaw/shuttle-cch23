use axum::{extract::Path, response::IntoResponse};
use tracing::info;

use crate::router::ResponseError;

pub async fn both_tasks(Path(x): Path<String>) -> Result<impl IntoResponse, ResponseError> {
    info!(?x);

    let sum = x
        .split('/')
        .map(|x| x.parse::<i32>().unwrap_or(0))
        .fold(0, |acc, x| acc ^ x);

    let result = sum.pow(3);

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use crate::router::router;

    use axum::http::StatusCode;
    use axum_test_helper::TestClient;

    #[rstest::rstest]
    #[case("4/8", "1728")]
    #[case("10", "1000")]
    #[case("4/5/8/10", "27")]
    #[tokio::test]
    async fn test_both_tasks(#[case] input: &str, #[case] expected: &str) {
        let router = router();
        let client = TestClient::new(router);
        let res = client.get(&format!("/1/{}", input)).send().await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await, expected);
    }
}
