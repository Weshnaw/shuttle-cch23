use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::router::ResponseError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    content: String,
}

pub async fn task_01(Json(content): Json<Content>) -> Result<impl IntoResponse, ResponseError> {
    info!(?content);
    Ok(format!(
        "\
<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {}
  </body>
</html>",
        content.content
    ))
}

pub async fn task_02(Json(content): Json<Content>) -> Result<impl IntoResponse, ResponseError> {
    let content = content.content;
    let content = html_escape::encode_text(&content);
    let content = content.replace('"', "&quot;");
    info!(?content);
    Ok(format!(
        "\
<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {}
  </body>
</html>",
        content
    ))
}
