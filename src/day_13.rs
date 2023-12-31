use std::sync::Arc;

use anyhow::Context;
use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tracing::info;

use crate::router::{self, Error};

pub async fn task_01(State(state): State<Arc<router::State>>) -> Result<impl IntoResponse, Error> {
    let sql = sqlx::query_scalar!("SELECT 20231213")
        .fetch_one(&state.pool)
        .await
        .context("Failed to select number")?
        .unwrap_or(0);

    Ok(sql.to_string())
}

pub async fn task_02_reset(
    State(state): State<Arc<router::State>>,
) -> Result<impl IntoResponse, Error> {
    let mut transaction = state
        .pool
        .begin()
        .await
        .context("Failed to init transaction")?;
    sqlx::query!("DROP TABLE IF EXISTS orders")
        .execute(&mut *transaction)
        .await
        .context("Failed to drop orders")?;
    sqlx::query!("CREATE TABLE orders (id INT PRIMARY KEY, region_id INT, gift_name VARCHAR(50), quantity INT)").execute(&mut *transaction).await.context("Failed to create orders")?;

    transaction
        .commit()
        .await
        .context("Failed to commit reset transaction")?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: i32,
    pub region_id: i32,
    pub gift_name: String,
    pub quantity: i32,
}

pub async fn task_02_orders(
    State(state): State<Arc<router::State>>,
    Json(orders): Json<Vec<Order>>,
) -> Result<impl IntoResponse, Error> {
    let mut transaction = state
        .pool
        .begin()
        .await
        .context("Failed to init transaction")?;

    info!(?orders);

    for order in orders {
        sqlx::query!(
            "INSERT into orders (id, region_id, gift_name, quantity) VALUES ($1, $2, $3, $4)",
            order.id,
            order.region_id,
            order.gift_name,
            order.quantity
        )
        .execute(&mut *transaction)
        .await
        .context("Failed to insert into orders")?;
    }

    transaction
        .commit()
        .await
        .context("Failed to commit orders insert transaction")?;

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct Total {
    total: i64,
}

pub async fn task_02_total(
    State(state): State<Arc<router::State>>,
) -> Result<impl IntoResponse, Error> {
    let total = sqlx::query_scalar!("SELECT SUM(quantity) FROM orders")
        .fetch_one(&state.pool)
        .await
        .context("Failed to select SUM")?
        .unwrap_or(0);
    info!(?total);

    Ok(Json(Total { total }))
}

#[derive(Serialize, Deserialize)]
pub struct Popular {
    popular: Option<String>,
}

pub async fn task_03_popular(
    State(state): State<Arc<router::State>>,
) -> Result<impl IntoResponse, Error> {
    let popular = sqlx::query!("SELECT gift_name FROM (SELECT gift_name, SUM(quantity) AS total FROM orders GROUP BY gift_name) AS q_one WHERE total = (SELECT MAX(total) FROM (SELECT gift_name, SUM(quantity) AS total FROM orders GROUP BY gift_name) AS q_two)")
        .fetch_one(&state.pool)
        .await.map(|r| r.gift_name).ok().flatten();

    info!(?popular);

    Ok(Json(Popular { popular }))
}
