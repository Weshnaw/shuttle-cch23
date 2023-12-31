use std::sync::Arc;

use anyhow::Context;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::router::{self, Error};

#[derive(Debug, Serialize, Deserialize)]
struct Pokemon {
    weight: i32,
}

pub async fn task_01(
    Path(number): Path<i32>,
    State(state): State<Arc<router::State>>,
) -> Result<impl IntoResponse, Error> {
    let poke: Pokemon = state
        .client
        .get(format!("https://pokeapi.co/api/v2/pokemon/{}", number))
        .send()
        .await
        .context("Failed to send pokeapi request")?
        .json()
        .await
        .context("Failed to parse response json")?;

    debug!(?poke);
    Ok((poke.weight as f32 / 10f32).to_string())
}

const GRAV: f32 = 2f32 * 9.825 * 10f32;

pub async fn task_02(
    Path(number): Path<i32>,
    State(state): State<Arc<router::State>>,
) -> Result<impl IntoResponse, Error> {
    let poke: Pokemon = state
        .client
        .get(format!("https://pokeapi.co/api/v2/pokemon/{}", number))
        .send()
        .await
        .context("Failed to send pokeapi request")?
        .json()
        .await
        .context("Failed to parse response json")?;

    Ok((GRAV.sqrt() * (poke.weight as f32 / 10f32)).to_string())
}
