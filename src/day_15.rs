use anyhow::Context;
use axum::{http::status::StatusCode, response::IntoResponse, Json};
use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::router::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    input: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    result: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

enum NoN {
    Naughty,
    Nice,
    ReasonedNaughty(String),
    SpecialStatusNaughty(StatusCode, String),
    ReasonedNice(String),
}

impl IntoResponse for NoN {
    fn into_response(self) -> axum::response::Response {
        use NoN::*;

        match self {
            Naughty => (
                StatusCode::BAD_REQUEST,
                Json(Response {
                    result: "naughty".to_string(),
                    reason: None,
                }),
            ),
            Nice => (
                StatusCode::OK,
                Json(Response {
                    result: "nice".to_string(),
                    reason: None,
                }),
            ),
            ReasonedNaughty(reason) => (
                StatusCode::BAD_REQUEST,
                Json(Response {
                    result: "naughty".to_string(),
                    reason: Some(reason),
                }),
            ),
            SpecialStatusNaughty(code, reason) => (
                code,
                Json(Response {
                    result: "naughty".to_string(),
                    reason: Some(reason),
                }),
            ),
            ReasonedNice(reason) => (
                StatusCode::OK,
                Json(Response {
                    result: "nice".to_string(),
                    reason: Some(reason),
                }),
            ),
        }
        .into_response()
    }
}

pub async fn task_01(Json(input): Json<Input>) -> Result<impl IntoResponse, Error> {
    let input = input.input.to_lowercase();

    info!(?input);

    let bad = Regex::new(r"(ab|cd|pq|xy)")
        .context("Failed to create bad pair regex")?
        .find(&input)
        .is_some();
    if bad {
        info!("not vowels");
        return Ok(NoN::Naughty);
    }

    let double = input
        .chars()
        .map_windows(|[a, b]| (*a, *b))
        .any(|(a, b)| a == b && a.is_ascii_alphabetic());
    if !double {
        info!("not doubled");
        return Ok(NoN::Naughty);
    }

    let vowels = Regex::new(r"[aeiouy]")
        .context("Failed to create vowel regex")?
        .find_iter(&input)
        .count();
    if vowels < 3 {
        info!("not vowels");
        return Ok(NoN::Naughty);
    }

    Ok(NoN::Nice)
}

pub async fn task_02(Json(input): Json<Input>) -> Result<impl IntoResponse, Error> {
    let input = input.input;
    info!(?input);

    if input.len() < 8 {
        return Ok(NoN::ReasonedNaughty("8 chars".to_string()));
    }

    let has_upper = Regex::new(r"[A-Z]")
        .context("Failed to create uppercase regex")?
        .find(&input)
        .is_some();
    if !has_upper {
        return Ok(NoN::ReasonedNaughty("more types of chars".to_string()));
    }

    let has_lower = Regex::new(r"[a-z]")
        .context("Failed to create lowercase regex")?
        .find(&input)
        .is_some();
    if !has_lower {
        return Ok(NoN::ReasonedNaughty("more types of chars".to_string()));
    }

    let digits = Regex::new(r"[0-9]")
        .context("Failed to create digits regex")?
        .find_iter(&input)
        .count();
    if digits == 0 {
        return Ok(NoN::ReasonedNaughty("more types of chars".to_string()));
    }

    if digits < 5 {
        return Ok(NoN::ReasonedNaughty("55555".to_string()));
    }

    let sum: i32 = Regex::new(r"[0-9]+")
        .context("Failed to create sums regex")?
        .find_iter(&input)
        .map(|ma| {
            let ma = ma.as_str();
            debug!(?ma);
            ma.parse::<i32>().unwrap_or(0)
        })
        .sum();
    info!(?sum);
    if sum != 2023 {
        return Ok(NoN::ReasonedNaughty("math is hard".to_string()));
    }

    let joy = Regex::new(r"j.*o.*y")
        .context("Failed to create joy regex")?
        .find(&input)
        .is_some();
    let not_joy_y = Regex::new(r"y.*j")
        .context("Failed to create not yj regex")?
        .find(&input)
        .is_some();
    let not_joy_o = Regex::new(r"o.*j")
        .context("Failed to create not oj regex")?
        .find(&input)
        .is_some();
    let not_joy_yo = Regex::new(r"y.*o")
        .context("Failed to create not yo regex")?
        .find(&input)
        .is_some();
    if !joy || not_joy_y || not_joy_o || not_joy_yo {
        return Ok(NoN::SpecialStatusNaughty(
            StatusCode::NOT_ACCEPTABLE,
            "not joyful enough".to_string(),
        ));
    }

    let r6 = input
        .chars()
        .map_windows(|[a, b, c]| (*a, *b, *c))
        .any(|(a, b, c)| a == c && a.is_ascii_alphabetic() && b.is_ascii_alphabetic());
    if !r6 {
        return Ok(NoN::SpecialStatusNaughty(
            StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
            "illegal: no sandwich".to_string(),
        ));
    }

    let r7 = Regex::new(r"[\u2980-\u2BFF]")
        .context("Failed to create r7 regex")?
        .find(&input)
        .is_some();
    if !r7 {
        return Ok(NoN::SpecialStatusNaughty(
            StatusCode::RANGE_NOT_SATISFIABLE,
            "outranged".to_string(),
        ));
    }

    let r8 = Regex::new(r"\p{Emoji_Presentation}")
        .context("Failed to create r8 regex")?
        .find(&input)
        .is_some();
    if !r8 {
        return Ok(NoN::SpecialStatusNaughty(
            StatusCode::UPGRADE_REQUIRED,
            "😳".to_string(),
        ));
    }

    let r8 = sha256::digest(input);
    if r8.chars().last().unwrap_or('_') != 'a' {
        return Ok(NoN::SpecialStatusNaughty(
            StatusCode::IM_A_TEAPOT,
            "not a coffee brewer".to_string(),
        ));
    }

    Ok(NoN::ReasonedNice("that's a nice password".to_string()))
}
