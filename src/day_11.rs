use std::{io::Cursor, num::Saturating};

use axum::{extract::Multipart, response::IntoResponse};
use image::io::Reader as ImageReader;
use tracing::info;

use crate::router::ResponseError;

pub async fn task_02(mut multipart: Multipart) -> Result<impl IntoResponse, ResponseError> {
    let mut bytes = Vec::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        info!("Length of `{}` is {} bytes", name, data.len());
        bytes.push(data);
    }

    let data = bytes.into_iter().flatten().collect::<Vec<_>>();
    let img = ImageReader::new(Cursor::new(data))
        .with_guessed_format()
        .expect("failed to guess format")
        .decode()
        .expect("failed to decode")
        .into_rgb16();

    let count = img
        .pixels()
        .filter(|p| (Saturating(p.0[0]) - Saturating(p.0[1])) > Saturating(p.0[2]))
        .count();

    Ok(count.to_string())
}
