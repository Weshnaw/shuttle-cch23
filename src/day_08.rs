use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::router::{self, ResponseError};

#[derive(Serialize, Deserialize)]
struct Pokemon {
    weight: i32,
}

pub async fn task_01(
    Path(number): Path<i32>,
    State(state): State<router::State>,
) -> Result<impl IntoResponse, ResponseError> {
    let poke: Pokemon = state
        .client
        .get(format!("https://pokeapi.co/api/v2/pokemon/{}", number))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    debug!("poke");
    Ok((poke.weight / 10).to_string())
}

const GRAV: f32 = 2f32 * 9.825 * 10f32;

pub async fn task_02(
    Path(number): Path<i32>,
    State(state): State<router::State>,
) -> Result<impl IntoResponse, ResponseError> {
    let poke: Pokemon = state
        .client
        .get(format!("https://pokeapi.co/api/v2/pokemon/{}", number))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    Ok((GRAV.sqrt() * (poke.weight / 10) as f32).to_string())
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
        let res = client.get("/8/weight/25").send().await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await, "6");
    }

    #[tokio::test]
    async fn test_02() {
        let router = router();
        let client = TestClient::new(router);
        let res = client.get("/8/drop/25").send().await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await, "84.10708");
    }
}
