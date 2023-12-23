use std::collections::HashMap;

use anyhow::Context;
use axum::{http::HeaderMap, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::router::Error;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Default)]
#[serde(default)]
struct BakeInput {
    recipe: HashMap<String, u64>,
    pantry: HashMap<String, u64>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct BakeOutput {
    cookies: u64,
    pantry: HashMap<String, u64>,
}

const COOKIE_HEADER: &str = "Cookie";
const RECIPE: usize = "recipe=".len();

pub async fn task_01(headers: HeaderMap) -> Result<impl IntoResponse, Error> {
    let cookie = headers
        .get(COOKIE_HEADER)
        .context("Heading not found")?
        .to_str()
        .context("Heading not string")?;
    info!(?cookie);
    let recipe = String::from_utf8(
        rbase64::decode(&cookie[RECIPE..])
            .with_context(|| format!("Unable to decode cookie: {}", cookie))?,
    )
    .context("Unable to convert decoded value to string")?;

    info!(?recipe);
    Ok(recipe)
}

pub async fn task_02(headers: HeaderMap) -> Result<impl IntoResponse, Error> {
    let cookie = headers
        .get(COOKIE_HEADER)
        .context("Heading not found")?
        .to_str()
        .context("Heading not string")?;
    debug!(?cookie);
    let base64 = &cookie[RECIPE..];
    debug!(?base64);
    let decoded = &String::from_utf8(
        rbase64::decode(base64).with_context(|| format!("Unable to decode cookie: {}", cookie))?,
    )
    .context("Unable to convert decoded value to string")?;

    debug!(?decoded);
    let recipe: BakeInput =
        serde_json::from_str(decoded).context("Unable to parse decoded to json")?;
    info!(?decoded);

    let max_cookies = recipe
        .recipe
        .iter()
        .filter_map(|(key, val)| {
            if val > &0 {
                Some(
                    recipe
                        .pantry
                        .get(key)
                        .map(|pantry| pantry / val)
                        .unwrap_or(0),
                )
            } else {
                None
            }
        })
        .min()
        .unwrap_or(0);
    info!(?max_cookies);

    let pantry = recipe
        .pantry
        .into_iter()
        .map(|(key, val)| {
            recipe
                .recipe
                .get(&key)
                .map(|rec| (key.clone(), val - (rec * max_cookies)))
                .unwrap_or((key, val))
        })
        .collect();

    info!(?pantry);
    Ok(Json(BakeOutput {
        cookies: max_cookies,
        pantry,
    }))
}
