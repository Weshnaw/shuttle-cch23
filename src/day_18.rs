use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tracing::info;

use crate::{
    day_13::Order,
    router::{self, ResponseError},
};

pub async fn task_01_reset(
    State(state): State<Arc<router::State>>,
) -> Result<impl IntoResponse, ResponseError> {
    let mut transaction = state.pool.begin().await.unwrap();

    sqlx::query!("DROP TABLE IF EXISTS regions")
        .execute(&mut *transaction)
        .await?;
    sqlx::query!("DROP TABLE IF EXISTS orders")
        .execute(&mut *transaction)
        .await?;
    sqlx::query!("CREATE TABLE regions (id INT PRIMARY KEY, name VARCHAR(50))")
        .execute(&mut *transaction)
        .await?;
    sqlx::query!("CREATE TABLE orders (id INT PRIMARY KEY, region_id INT, gift_name VARCHAR(50), quantity INT)").execute(&mut *transaction).await?;

    transaction.commit().await?;
    info!("Day 18 Reset Called");
    Ok(())
}

pub async fn task_01_orders(
    State(state): State<Arc<router::State>>,
    Json(orders): Json<Vec<Order>>,
) -> Result<impl IntoResponse, ResponseError> {
    let mut transaction = state.pool.begin().await?;

    info!(?orders);

    for order in orders {
        sqlx::query!(
            r#"
            INSERT INTO orders (
                id, 
                region_id, 
                gift_name, 
                quantity
            ) VALUES (
                $1, 
                $2, 
                $3, 
                $4
            )"#,
            order.id,
            order.region_id,
            order.gift_name,
            order.quantity
        )
        .execute(&mut *transaction)
        .await?;
    }

    transaction.commit().await?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    id: i32,
    name: String,
}

pub async fn task_01_regions(
    State(state): State<Arc<router::State>>,
    Json(regions): Json<Vec<Region>>,
) -> Result<impl IntoResponse, ResponseError> {
    let mut transaction = state.pool.begin().await?;

    info!(?regions);

    for region in regions {
        sqlx::query!(
            r#"
            INSERT INTO regions (
                id, 
                name
            ) VALUES (
                $1, 
                $2
            )"#,
            region.id,
            region.name,
        )
        .execute(&mut *transaction)
        .await?;
    }

    transaction.commit().await?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RegionResult {
    #[serde(rename = "region")]
    name: Option<String>,
    total: Option<i64>,
}

pub async fn task_01_total(
    State(state): State<Arc<router::State>>,
) -> Result<impl IntoResponse, ResponseError> {
    let total = sqlx::query_as!(
        RegionResult,
        r#"
        SELECT r.name, o.total 
        FROM (
            SELECT 
                region_id, 
                SUM(quantity) AS total 
            FROM orders 
            GROUP BY region_id
        ) AS o 
        LEFT JOIN regions 
            AS r ON r.id = o.region_id 
        ORDER BY r.name
        "#
    )
    .fetch_all(&state.pool)
    .await?;

    let total = total
        .into_iter()
        .filter(|r| r.total.is_some() && r.name.is_some())
        .collect::<Vec<_>>();
    info!(?total);

    Ok(Json(total))
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TopResponse {
    region: Option<String>,
    top_gifts: Option<Vec<String>>,
}

pub async fn task_02(
    Path(number): Path<i64>,
    State(state): State<Arc<router::State>>,
) -> Result<impl IntoResponse, ResponseError> {
    let total = sqlx::query_as!(
        TopResponse,
        r#"
        SELECT 
            r.name AS region,
            ARRAY_REMOVE(ARRAY_AGG(o.gift_name), NULL) AS top_gifts
        FROM (
            SELECT 
                region_id, 
                gift_name,
                total
        		FROM (
                    SELECT 
                        region_id,
                        gift_name,
                        SUM(quantity) AS total,
                        ROW_NUMBER() OVER (
                            PARTITION BY region_id
                            ORDER BY 
                                SUM(quantity) DESC, 
                                gift_name ASC
                        ) AS r_num
                    FROM orders
                    GROUP BY 
                        region_id, 
                        gift_name
                ) AS o
            WHERE o.r_num <= $1
        ) AS o
        RIGHT JOIN regions AS r 
            ON r.id = o.region_id
        GROUP BY r.name
        ORDER BY r.name
        "#,
        number
    )
    .fetch_all(&state.pool)
    .await?;

    info!(?total);

    Ok(Json(total))
}
