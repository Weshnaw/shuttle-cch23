use std::{io::Cursor, num::Saturating};

use axum::{extract::Multipart, response::IntoResponse};
use image::io::Reader as ImageReader;
use tracing::info;

use crate::router::ResponseError;

pub async fn task_02(mut multipart: Multipart) -> Result<impl IntoResponse, ResponseError> {
    let mut count = 0;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        info!("Length of `{}` is {} bytes", name, data.len());

        // I felt like specifically checking for image
        if name == "image" {
            let img = ImageReader::new(Cursor::new(data))
                .with_guessed_format()
                .expect("failed to guess format")
                .decode()
                .expect("failed to decode")
                .into_rgb16();
            // I also wanted to parse instances where multiple images are sent
            count += img
                .pixels()
                .filter(|p| (Saturating(p.0[0]) - Saturating(p.0[1])) > Saturating(p.0[2]))
                .count();
        }
    }

    Ok(count.to_string())
}
