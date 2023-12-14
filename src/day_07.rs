use std::collections::HashMap;

use axum::{http::HeaderMap, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::router::ResponseError;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct BakeInput {
    #[serde(default)]
    recipe: HashMap<String, u64>,
    #[serde(default)]
    pantry: HashMap<String, u64>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct BakeOutput {
    cookies: u64,
    pantry: HashMap<String, u64>,
}

const COOKIE_HEADER: &str = "Cookie";

pub async fn task_01(headers: HeaderMap) -> Result<impl IntoResponse, ResponseError> {
    let cookie = headers
        .get(COOKIE_HEADER)
        .ok_or(ResponseError::HeadingNotFound(COOKIE_HEADER.to_string()))?;
    info!(?cookie);
    let recipe = String::from_utf8(
        rbase64::decode(&cookie.to_str().expect("failed to str cookie")["recipe=".len()..])
            .map_err(|_| ResponseError::Base64DecodeError)?,
    )?;

    info!(?recipe);
    Ok(recipe)
}

pub async fn task_02(headers: HeaderMap) -> Result<impl IntoResponse, ResponseError> {
    let cookie = headers
        .get(COOKIE_HEADER)
        .ok_or(ResponseError::HeadingNotFound(COOKIE_HEADER.to_string()))?;

    debug!(?cookie);
    let base64 = &cookie.to_str()?["recipe=".len()..];
    debug!(?base64);
    let decoded =
        &String::from_utf8(rbase64::decode(base64).map_err(|_| ResponseError::Base64DecodeError)?)?;
    debug!(?decoded);
    let recipe: BakeInput = serde_json::from_str(decoded)?;
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
